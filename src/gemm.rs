use std::alloc::{alloc, dealloc, Layout};

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
use std::arch::is_aarch64_feature_detected;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::x86_64::*;

// ============= Cache Configuration Parameters =============
// These should be tuned for your target architecture

// Level 1 cache parameters (typically 32KB)
const L1_CACHE_SIZE: usize = 32 * 1024;

// Level 2 cache parameters (typically 256KB-1MB)
const L2_CACHE_SIZE: usize = 256 * 1024;

// Level 3 cache parameters (typically 8MB-32MB)
const L3_CACHE_SIZE: usize = 8 * 1024 * 1024;

// Register blocking parameters for AVX2 (f64)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const MR_F64: usize = 6; // Number of rows in microkernel
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const NR_F64: usize = 8; // Number of columns in microkernel

// Register blocking parameters for NEON (f64)
#[cfg(target_arch = "aarch64")]
const MR_F64: usize = 8;
#[cfg(target_arch = "aarch64")]
const NR_F64: usize = 6;

// Default fallback values
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
const MR_F64: usize = 4;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
const NR_F64: usize = 4;

// Register blocking for f32 (typically double the f64 values)
const MR_F32: usize = MR_F64 * 2;
const NR_F32: usize = NR_F64 * 2;

// Cache blocking parameters (these are computed based on cache sizes)
fn compute_cache_params_f64() -> (usize, usize, usize) {
    let kc = 256; // Inner dimension blocking
    let mc = ((L2_CACHE_SIZE / 8) - kc * NR_F64) / kc;
    let mc = (mc / MR_F64) * MR_F64; // Round down to multiple of MR

    let nc = ((L3_CACHE_SIZE / 8) - mc * kc) / kc;
    let nc = (nc / NR_F64) * NR_F64; // Round down to multiple of NR

    (kc.min(512), mc.min(256), nc.min(1024))
}

fn compute_cache_params_f32() -> (usize, usize, usize) {
    let kc = 512;
    let mc = ((L2_CACHE_SIZE / 4) - kc * NR_F32) / kc;
    let mc = (mc / MR_F32) * MR_F32;

    let nc = ((L3_CACHE_SIZE / 4) - mc * kc) / kc;
    let nc = (nc / NR_F32) * NR_F32;

    (kc.min(1024), mc.min(512), nc.min(2048))
}

// ============= Packing Functions =============

/// Pack matrix A into column-major blocks for L2 cache
unsafe fn pack_a_f64(
    pack_a: *mut f64,
    a: *const f64,
    lda: usize,
    mc: usize,
    kc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_a;

    for i in (0..mc).step_by(MR_F64) {
        let remaining_rows = (mc - i).min(MR_F64);

        for k in 0..kc {
            for ir in 0..MR_F64 {
                if ir < remaining_rows {
                    *pack_ptr = *a.add((row_offset + i + ir) * lda + col_offset + k);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

/// Pack matrix B into row-major blocks for L3 cache
unsafe fn pack_b_f64(
    pack_b: *mut f64,
    b: *const f64,
    ldb: usize,
    kc: usize,
    nc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_b;

    for j in (0..nc).step_by(NR_F64) {
        let remaining_cols = (nc - j).min(NR_F64);

        for k in 0..kc {
            for jr in 0..NR_F64 {
                if jr < remaining_cols {
                    *pack_ptr = *b.add((row_offset + k) * ldb + col_offset + j + jr);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

/// Pack transposed matrix A (reading A^T) into column-major blocks
unsafe fn pack_a_trans_f64(
    pack_a: *mut f64,
    a: *const f64,
    lda: usize,
    mc: usize,
    kc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_a;

    for i in (0..mc).step_by(MR_F64) {
        let remaining_rows = (mc - i).min(MR_F64);

        for k in 0..kc {
            for ir in 0..MR_F64 {
                if ir < remaining_rows {
                    // Read A^T: swap indices
                    *pack_ptr = *a.add((col_offset + k) * lda + row_offset + i + ir);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

/// Pack transposed matrix B (reading B^T) into row-major blocks
unsafe fn pack_b_trans_f64(
    pack_b: *mut f64,
    b: *const f64,
    ldb: usize,
    kc: usize,
    nc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_b;

    for j in (0..nc).step_by(NR_F64) {
        let remaining_cols = (nc - j).min(NR_F64);

        for k in 0..kc {
            for jr in 0..NR_F64 {
                if jr < remaining_cols {
                    // Read B^T: swap indices
                    *pack_ptr = *b.add((col_offset + j + jr) * ldb + row_offset + k);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

// ============= f32 Packing Functions =============

unsafe fn pack_a_f32(
    pack_a: *mut f32,
    a: *const f32,
    lda: usize,
    mc: usize,
    kc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_a;

    for i in (0..mc).step_by(MR_F32) {
        let remaining_rows = (mc - i).min(MR_F32);

        for k in 0..kc {
            for ir in 0..MR_F32 {
                if ir < remaining_rows {
                    *pack_ptr = *a.add((row_offset + i + ir) * lda + col_offset + k);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

unsafe fn pack_b_f32(
    pack_b: *mut f32,
    b: *const f32,
    ldb: usize,
    kc: usize,
    nc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_b;

    for j in (0..nc).step_by(NR_F32) {
        let remaining_cols = (nc - j).min(NR_F32);

        for k in 0..kc {
            for jr in 0..NR_F32 {
                if jr < remaining_cols {
                    *pack_ptr = *b.add((row_offset + k) * ldb + col_offset + j + jr);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

unsafe fn pack_a_trans_f32(
    pack_a: *mut f32,
    a: *const f32,
    lda: usize,
    mc: usize,
    kc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_a;

    for i in (0..mc).step_by(MR_F32) {
        let remaining_rows = (mc - i).min(MR_F32);

        for k in 0..kc {
            for ir in 0..MR_F32 {
                if ir < remaining_rows {
                    *pack_ptr = *a.add((col_offset + k) * lda + row_offset + i + ir);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

unsafe fn pack_b_trans_f32(
    pack_b: *mut f32,
    b: *const f32,
    ldb: usize,
    kc: usize,
    nc: usize,
    row_offset: usize,
    col_offset: usize,
) {
    let mut pack_ptr = pack_b;

    for j in (0..nc).step_by(NR_F32) {
        let remaining_cols = (nc - j).min(NR_F32);

        for k in 0..kc {
            for jr in 0..NR_F32 {
                if jr < remaining_cols {
                    *pack_ptr = *b.add((col_offset + j + jr) * ldb + row_offset + k);
                } else {
                    *pack_ptr = 0.0;
                }
                pack_ptr = pack_ptr.add(1);
            }
        }
    }
}

// ============= Microkernel =============

/// High-performance microkernel for f64 GEMM
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn gemm_microkernel_f64_avx2(
    k: usize,
    alpha: f64,
    pack_a: *const f64,
    pack_b: *const f64,
    beta: f64,
    c: *mut f64,
    ldc: usize,
    mr: usize,
    nr: usize,
) {
    // Initialize accumulators for the MR×NR tile
    let mut c_regs = [[_mm256_setzero_pd(); NR_F64 / 4]; MR_F64];

    let alpha_vec = _mm256_set1_pd(alpha);
    let beta_vec = _mm256_set1_pd(beta);

    // Main accumulation loop
    for kk in 0..k {
        // Load column of A (MR elements)
        let a_col = [
            _mm256_broadcast_sd(pack_a.add(kk * MR_F64)),
            _mm256_broadcast_sd(pack_a.add(kk * MR_F64 + 1)),
            _mm256_broadcast_sd(pack_a.add(kk * MR_F64 + 2)),
            _mm256_broadcast_sd(pack_a.add(kk * MR_F64 + 3)),
            _mm256_broadcast_sd(pack_a.add(kk * MR_F64 + 4)),
            _mm256_broadcast_sd(pack_a.add(kk * MR_F64 + 5)),
        ];

        // Load row of B (NR elements) and compute outer product
        for j in 0..NR_F64 / 4 {
            let b_vec = _mm256_loadu_pd(pack_b.add(kk * NR_F64 + j * 4));

            for i in 0..MR_F64 {
                c_regs[i][j] = _mm256_fmadd_pd(a_col[i], b_vec, c_regs[i][j]);
            }
        }
    }

    // Store results back to C with alpha and beta scaling
    for i in 0..mr {
        for j in 0..nr / 4 {
            let c_ptr = c.add(i * ldc + j * 4);
            let c_vec = _mm256_loadu_pd(c_ptr);
            let result = _mm256_fmadd_pd(alpha_vec, c_regs[i][j], _mm256_mul_pd(beta_vec, c_vec));
            _mm256_storeu_pd(c_ptr, result);
        }

        // Handle remaining columns
        for j in (nr / 4) * 4..nr {
            let c_ptr = c.add(i * ldc + j);
            let c_val = *c_ptr;
            // Extract the correct element from the appropriate accumulator
            let vec_idx = j / 4;
            let lane_idx = j % 4;
            let acc_vec = c_regs[i][vec_idx];

            // Extract all 4 elements to an array
            let mut acc_array: [f64; 4] = [0.0; 4];
            _mm256_storeu_pd(acc_array.as_mut_ptr(), acc_vec);
            let acc_val = acc_array[lane_idx];

            *c_ptr = alpha * acc_val + beta * c_val;
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn gemm_microkernel_f64_neon(
    k: usize,
    alpha: f64,
    pack_a: *const f64,
    pack_b: *const f64,
    beta: f64,
    c: *mut f64,
    ldc: usize,
    mr: usize,
    nr: usize,
) {
    // Initialize accumulators
    let mut c_regs = [[vdupq_n_f64(0.0); NR_F64 / 2]; MR_F64];

    // Main accumulation loop
    for kk in 0..k {
        // Load column of A
        for i in 0..MR_F64 {
            let a_val = *pack_a.add(kk * MR_F64 + i);
            let a_vec = vdupq_n_f64(a_val);

            // Multiply with row of B and accumulate
            for j in 0..NR_F64 / 2 {
                let b_vec = vld1q_f64(pack_b.add(kk * NR_F64 + j * 2));
                c_regs[i][j] = vfmaq_f64(c_regs[i][j], a_vec, b_vec);
            }
        }
    }

    // Store results
    for i in 0..mr {
        for j in 0..nr / 2 {
            let c_ptr = c.add(i * ldc + j * 2);
            let c_vec = vld1q_f64(c_ptr);
            let result = vaddq_f64(vmulq_n_f64(c_regs[i][j], alpha), vmulq_n_f64(c_vec, beta));
            vst1q_f64(c_ptr, result);
        }

        // Handle odd columns
        if nr % 2 == 1 {
            let j = nr - 1;
            let c_ptr = c.add(i * ldc + j);
            let acc_val = vgetq_lane_f64(c_regs[i][j / 2], 0);
            *c_ptr = alpha * acc_val + beta * *c_ptr;
        }
    }
}

// Fallback scalar microkernel
unsafe fn gemm_microkernel_f64_scalar(
    k: usize,
    alpha: f64,
    pack_a: *const f64,
    pack_b: *const f64,
    beta: f64,
    c: *mut f64,
    ldc: usize,
    mr: usize,
    nr: usize,
) {
    // Simple nested loops with accumulation
    for i in 0..mr {
        for j in 0..nr {
            let mut sum = 0.0;
            for kk in 0..k {
                sum += *pack_a.add(kk * MR_F64 + i) * *pack_b.add(kk * NR_F64 + j);
            }
            let c_ptr = c.add(i * ldc + j);
            *c_ptr = alpha * sum + beta * *c_ptr;
        }
    }
}

// ============= f32 Microkernels =============

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn gemm_microkernel_f32_avx2(
    k: usize,
    alpha: f32,
    pack_a: *const f32,
    pack_b: *const f32,
    beta: f32,
    c: *mut f32,
    ldc: usize,
    mr: usize,
    nr: usize,
) {
    // We can handle 12x16 with AVX2 for f32
    const MR_MAX: usize = 12;
    const NR_STEP: usize = 8;

    // Initialize accumulators
    let mut c_regs = [[_mm256_setzero_ps(); 2]; MR_MAX];

    let alpha_vec = _mm256_set1_ps(alpha);
    let beta_vec = _mm256_set1_ps(beta);

    // Main accumulation loop
    for kk in 0..k {
        let nr_blocks = (nr + NR_STEP - 1) / NR_STEP;

        for block in 0..nr_blocks.min(2) {
            let b_offset = kk * NR_F32 + block * NR_STEP;
            let b_vec = if block * NR_STEP + NR_STEP <= nr {
                _mm256_loadu_ps(pack_b.add(b_offset))
            } else {
                let mut temp = [0.0f32; 8];
                for j in 0..(nr - block * NR_STEP) {
                    temp[j] = *pack_b.add(b_offset + j);
                }
                _mm256_loadu_ps(temp.as_ptr())
            };

            for i in 0..mr.min(MR_MAX) {
                let a_val = *pack_a.add(kk * MR_F32 + i);
                let a_vec = _mm256_set1_ps(a_val);
                c_regs[i][block] = _mm256_fmadd_ps(a_vec, b_vec, c_regs[i][block]);
            }
        }
    }

    // Store results
    for i in 0..mr.min(MR_MAX) {
        let c_row_ptr = c.add(i * ldc);

        let full_blocks = nr / NR_STEP;
        for block in 0..full_blocks.min(2) {
            let c_ptr = c_row_ptr.add(block * NR_STEP);
            let c_vec = _mm256_loadu_ps(c_ptr);
            let result =
                _mm256_fmadd_ps(alpha_vec, c_regs[i][block], _mm256_mul_ps(beta_vec, c_vec));
            _mm256_storeu_ps(c_ptr, result);
        }

        let remainder = nr % NR_STEP;
        if remainder > 0 && full_blocks < 2 {
            let block = full_blocks;
            let c_ptr = c_row_ptr.add(block * NR_STEP);

            let acc_vec = c_regs[i][block];
            let mut acc_array: [f32; 8] = std::mem::transmute(acc_vec);

            for j in 0..remainder {
                let c_val = *c_ptr.add(j);
                *c_ptr.add(j) = alpha * acc_array[j] + beta * c_val;
            }
        }
    }

    // Handle remaining rows
    if mr > MR_MAX {
        for i in MR_MAX..mr {
            for j in 0..nr {
                let mut sum = 0.0;
                for kk in 0..k {
                    sum += *pack_a.add(kk * MR_F32 + i) * *pack_b.add(kk * NR_F32 + j);
                }
                let c_ptr = c.add(i * ldc + j);
                *c_ptr = alpha * sum + beta * *c_ptr;
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn gemm_microkernel_f32_neon(
    k: usize,
    alpha: f32,
    pack_a: *const f32,
    pack_b: *const f32,
    beta: f32,
    c: *mut f32,
    ldc: usize,
    mr: usize,
    nr: usize,
) {
    const MR_MAX: usize = 12;
    const NR_STEP: usize = 4;

    let mut c_regs = [[vdupq_n_f32(0.0); 4]; MR_MAX];

    let alpha_scalar = alpha;
    let beta_scalar = beta;

    for kk in 0..k {
        let nr_blocks = (nr + NR_STEP - 1) / NR_STEP;

        for block in 0..nr_blocks.min(4) {
            let b_offset = kk * NR_F32 + block * NR_STEP;

            let b_vec = if block * NR_STEP + NR_STEP <= nr {
                vld1q_f32(pack_b.add(b_offset))
            } else {
                let mut temp = [0.0f32; 4];
                for j in 0..(nr - block * NR_STEP) {
                    temp[j] = *pack_b.add(b_offset + j);
                }
                vld1q_f32(temp.as_ptr())
            };

            for i in 0..mr.min(MR_MAX) {
                let a_val = *pack_a.add(kk * MR_F32 + i);
                let a_vec = vdupq_n_f32(a_val);
                c_regs[i][block] = vfmaq_f32(c_regs[i][block], a_vec, b_vec);
            }
        }
    }

    for i in 0..mr.min(MR_MAX) {
        let c_row_ptr = c.add(i * ldc);

        let full_blocks = nr / NR_STEP;
        for block in 0..full_blocks.min(4) {
            let c_ptr = c_row_ptr.add(block * NR_STEP);
            let c_vec = vld1q_f32(c_ptr);
            let result = vaddq_f32(
                vmulq_n_f32(c_regs[i][block], alpha_scalar),
                vmulq_n_f32(c_vec, beta_scalar),
            );
            vst1q_f32(c_ptr, result);
        }

        let remainder = nr % NR_STEP;
        if remainder > 0 && full_blocks < 4 {
            let block = full_blocks;
            let c_ptr = c_row_ptr.add(block * NR_STEP);

            // Extract all lanes to an array since vgetq_lane_f32 needs compile-time constant
            let acc_vec = c_regs[i][block];
            let acc_array = [
                vgetq_lane_f32(acc_vec, 0),
                vgetq_lane_f32(acc_vec, 1),
                vgetq_lane_f32(acc_vec, 2),
                vgetq_lane_f32(acc_vec, 3),
            ];

            for j in 0..remainder {
                let c_val = *c_ptr.add(j);
                *c_ptr.add(j) = alpha_scalar * acc_array[j] + beta_scalar * c_val;
            }
        }
    }

    if mr > MR_MAX {
        for i in MR_MAX..mr {
            for j in 0..nr {
                let mut sum = 0.0;
                for kk in 0..k {
                    sum += *pack_a.add(kk * MR_F32 + i) * *pack_b.add(kk * NR_F32 + j);
                }
                let c_ptr = c.add(i * ldc + j);
                *c_ptr = alpha * sum + beta * *c_ptr;
            }
        }
    }
}

unsafe fn gemm_microkernel_f32_scalar(
    k: usize,
    alpha: f32,
    pack_a: *const f32,
    pack_b: *const f32,
    beta: f32,
    c: *mut f32,
    ldc: usize,
    mr: usize,
    nr: usize,
) {
    for i in 0..mr {
        for j in 0..nr {
            let mut sum = 0.0f32;
            for kk in 0..k {
                sum += *pack_a.add(kk * MR_F32 + i) * *pack_b.add(kk * NR_F32 + j);
            }
            let c_ptr = c.add(i * ldc + j);
            *c_ptr = alpha * sum + beta * *c_ptr;
        }
    }
}

// ============= Main GEMM Function =============

/// General matrix multiplication: C = alpha * A * B + beta * C
pub fn dgemm(
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) {
    assert!(a.len() >= m * lda || a.len() >= k * lda);
    assert!(b.len() >= k * ldb || b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f64();

    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 8, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 8, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f64 };
    let pack_b = unsafe { alloc(layout_b) as *mut f64 };

    let microkernel: unsafe fn(
        usize,
        f64,
        *const f64,
        *const f64,
        f64,
        *mut f64,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f64_avx2;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f64_neon;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f64_scalar;
    }

    // Main GEMM loops following BLIS algorithm
    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                pack_b_f64(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    pack_a_f64(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F64) {
                        let nr = NR_F64.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F64) {
                            let mr = MR_F64.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

/// GEMM with transposed A: C = alpha * A^T * B + beta * C
pub fn dgemm_tn(
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) {
    assert!(a.len() >= k * lda);
    assert!(b.len() >= k * ldb || b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f64();

    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 8, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 8, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f64 };
    let pack_b = unsafe { alloc(layout_b) as *mut f64 };

    let microkernel: unsafe fn(
        usize,
        f64,
        *const f64,
        *const f64,
        f64,
        *mut f64,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f64_avx2;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f64_neon;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f64_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                pack_b_f64(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    // Use transposed packing for A
                    pack_a_trans_f64(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F64) {
                        let nr = NR_F64.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F64) {
                            let mr = MR_F64.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

/// GEMM with transposed B: C = alpha * A * B^T + beta * C
pub fn dgemm_nt(
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) {
    assert!(a.len() >= m * lda || a.len() >= k * lda);
    assert!(b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f64();

    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 8, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 8, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f64 };
    let pack_b = unsafe { alloc(layout_b) as *mut f64 };

    let microkernel: unsafe fn(
        usize,
        f64,
        *const f64,
        *const f64,
        f64,
        *mut f64,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f64_avx2;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f64_neon;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f64_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                // Use transposed packing for B
                pack_b_trans_f64(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    pack_a_f64(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F64) {
                        let nr = NR_F64.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F64) {
                            let mr = MR_F64.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

/// GEMM with both transposed: C = alpha * A^T * B^T + beta * C
pub fn dgemm_tt(
    m: usize,
    n: usize,
    k: usize,
    alpha: f64,
    a: &[f64],
    lda: usize,
    b: &[f64],
    ldb: usize,
    beta: f64,
    c: &mut [f64],
    ldc: usize,
) {
    assert!(a.len() >= k * lda);
    assert!(b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f64();

    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 8, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 8, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f64 };
    let pack_b = unsafe { alloc(layout_b) as *mut f64 };

    let microkernel: unsafe fn(
        usize,
        f64,
        *const f64,
        *const f64,
        f64,
        *mut f64,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f64_avx2;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f64_neon;
        } else {
            microkernel = gemm_microkernel_f64_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f64_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                // Use transposed packing for B
                pack_b_trans_f64(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    // Use transposed packing for A
                    pack_a_trans_f64(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F64) {
                        let nr = NR_F64.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F64) {
                            let mr = MR_F64.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

// ============= f32 Main GEMM Functions =============

pub fn sgemm(
    m: usize,
    n: usize,
    k: usize,
    alpha: f32,
    a: &[f32],
    lda: usize,
    b: &[f32],
    ldb: usize,
    beta: f32,
    c: &mut [f32],
    ldc: usize,
) {
    assert!(a.len() >= m * lda || a.len() >= k * lda);
    assert!(b.len() >= k * ldb || b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f32();

    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 4, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 4, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f32 };
    let pack_b = unsafe { alloc(layout_b) as *mut f32 };

    let microkernel: unsafe fn(
        usize,
        f32,
        *const f32,
        *const f32,
        f32,
        *mut f32,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f32_avx2;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f32_neon;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f32_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                pack_b_f32(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    pack_a_f32(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F32) {
                        let nr = NR_F32.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F32) {
                            let mr = MR_F32.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

pub fn sgemm_tn(
    m: usize,
    n: usize,
    k: usize,
    alpha: f32,
    a: &[f32],
    lda: usize,
    b: &[f32],
    ldb: usize,
    beta: f32,
    c: &mut [f32],
    ldc: usize,
) {
    assert!(a.len() >= k * lda);
    assert!(b.len() >= k * ldb || b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f32();
    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 4, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 4, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f32 };
    let pack_b = unsafe { alloc(layout_b) as *mut f32 };

    let microkernel: unsafe fn(
        usize,
        f32,
        *const f32,
        *const f32,
        f32,
        *mut f32,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f32_avx2;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f32_neon;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f32_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                pack_b_f32(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    pack_a_trans_f32(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F32) {
                        let nr = NR_F32.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F32) {
                            let mr = MR_F32.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

pub fn sgemm_nt(
    m: usize,
    n: usize,
    k: usize,
    alpha: f32,
    a: &[f32],
    lda: usize,
    b: &[f32],
    ldb: usize,
    beta: f32,
    c: &mut [f32],
    ldc: usize,
) {
    assert!(a.len() >= m * lda || a.len() >= k * lda);
    assert!(b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f32();
    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 4, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 4, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f32 };
    let pack_b = unsafe { alloc(layout_b) as *mut f32 };

    let microkernel: unsafe fn(
        usize,
        f32,
        *const f32,
        *const f32,
        f32,
        *mut f32,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f32_avx2;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f32_neon;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f32_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                pack_b_trans_f32(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    pack_a_f32(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F32) {
                        let nr = NR_F32.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F32) {
                            let mr = MR_F32.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

pub fn sgemm_tt(
    m: usize,
    n: usize,
    k: usize,
    alpha: f32,
    a: &[f32],
    lda: usize,
    b: &[f32],
    ldb: usize,
    beta: f32,
    c: &mut [f32],
    ldc: usize,
) {
    assert!(a.len() >= k * lda);
    assert!(b.len() >= n * ldb);
    assert!(c.len() >= m * ldc);

    if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
        return;
    }

    if alpha == 0.0 {
        for i in 0..m {
            for j in 0..n {
                c[i * ldc + j] *= beta;
            }
        }
        return;
    }

    let (kc, mc, nc) = compute_cache_params_f32();
    let pack_a_size = mc * kc;
    let pack_b_size = kc * nc;

    let layout_a = Layout::from_size_align(pack_a_size * 4, 64).unwrap();
    let layout_b = Layout::from_size_align(pack_b_size * 4, 64).unwrap();

    let pack_a = unsafe { alloc(layout_a) as *mut f32 };
    let pack_b = unsafe { alloc(layout_b) as *mut f32 };

    let microkernel: unsafe fn(
        usize,
        f32,
        *const f32,
        *const f32,
        f32,
        *mut f32,
        usize,
        usize,
        usize,
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            microkernel = gemm_microkernel_f32_avx2;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            microkernel = gemm_microkernel_f32_neon;
        } else {
            microkernel = gemm_microkernel_f32_scalar;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        microkernel = gemm_microkernel_f32_scalar;
    }

    for jc in (0..n).step_by(nc) {
        let nc_cur = nc.min(n - jc);

        for pc in (0..k).step_by(kc) {
            let kc_cur = kc.min(k - pc);
            let beta_use = if pc == 0 { beta } else { 1.0 };

            unsafe {
                pack_b_trans_f32(pack_b, b.as_ptr(), ldb, kc_cur, nc_cur, pc, jc);
            }

            for ic in (0..m).step_by(mc) {
                let mc_cur = mc.min(m - ic);

                unsafe {
                    pack_a_trans_f32(pack_a, a.as_ptr(), lda, mc_cur, kc_cur, ic, pc);
                }

                unsafe {
                    for jr in (0..nc_cur).step_by(NR_F32) {
                        let nr = NR_F32.min(nc_cur - jr);

                        for ir in (0..mc_cur).step_by(MR_F32) {
                            let mr = MR_F32.min(mc_cur - ir);

                            let c_ptr = c.as_mut_ptr().add((ic + ir) * ldc + jc + jr);
                            let a_ptr = pack_a.add(ir * kc_cur);
                            let b_ptr = pack_b.add(jr * kc_cur);

                            microkernel(kc_cur, alpha, a_ptr, b_ptr, beta_use, c_ptr, ldc, mr, nr);
                        }
                    }
                }
            }
        }
    }

    unsafe {
        dealloc(pack_a as *mut u8, layout_a);
        dealloc(pack_b as *mut u8, layout_b);
    }
}

// ============= Simple matmul interface =============

/// Simple matrix multiplication: C = A * B
pub fn matmul_f64(m: usize, n: usize, k: usize, a: &[f64], b: &[f64]) -> Vec<f64> {
    assert_eq!(a.len(), m * k);
    assert_eq!(b.len(), k * n);

    let mut c = vec![0.0; m * n];
    dgemm(m, n, k, 1.0, a, k, b, n, 0.0, &mut c, n);
    c
}

pub fn matmul_f32(m: usize, n: usize, k: usize, a: &[f32], b: &[f32]) -> Vec<f32> {
    assert_eq!(a.len(), m * k);
    assert_eq!(b.len(), k * n);

    let mut c = vec![0.0; m * n];
    sgemm(m, n, k, 1.0, a, k, b, n, 0.0, &mut c, n);
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn naive_gemm(
        m: usize,
        n: usize,
        k: usize,
        alpha: f64,
        a: &[f64],
        lda: usize,
        b: &[f64],
        ldb: usize,
        beta: f64,
        c: &mut [f64],
        ldc: usize,
    ) {
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for kk in 0..k {
                    sum += a[i * lda + kk] * b[kk * ldb + j];
                }
                c[i * ldc + j] = alpha * sum + beta * c[i * ldc + j];
            }
        }
    }

    fn naive_gemm_f32(
        m: usize,
        n: usize,
        k: usize,
        alpha: f32,
        a: &[f32],
        lda: usize,
        b: &[f32],
        ldb: usize,
        beta: f32,
        c: &mut [f32],
        ldc: usize,
    ) {
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for kk in 0..k {
                    sum += a[i * lda + kk] * b[kk * ldb + j];
                }
                c[i * ldc + j] = alpha * sum + beta * c[i * ldc + j];
            }
        }
    }

    #[test]
    fn test_gemm_small() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; // 2x3
        let b = vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]; // 3x2
        let mut c = vec![0.0; 4]; // 2x2
        let mut c_ref = vec![0.0; 4];

        dgemm(2, 2, 3, 1.0, &a, 3, &b, 2, 0.0, &mut c, 2);
        naive_gemm(2, 2, 3, 1.0, &a, 3, &b, 2, 0.0, &mut c_ref, 2);

        for i in 0..4 {
            assert_abs_diff_eq!(c[i], c_ref[i], epsilon = 1e-10);
        }
    }

    #[test]
    fn test_sgemm_small() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b = vec![7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0];
        let mut c = vec![0.0f32; 4];
        let mut c_ref = vec![0.0f32; 4];

        sgemm(2, 2, 3, 1.0, &a, 3, &b, 2, 0.0, &mut c, 2);
        naive_gemm_f32(2, 2, 3, 1.0, &a, 3, &b, 2, 0.0, &mut c_ref, 2);

        for i in 0..4 {
            assert_abs_diff_eq!(c[i], c_ref[i], epsilon = 1e-6);
        }
    }
}
