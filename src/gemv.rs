use crate::dot32::sdot;
use crate::dot64::ddot;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
use std::arch::is_aarch64_feature_detected;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::x86_64::*;

// ============= f64 Implementation =============

/// Computes y = alpha * A * x + beta * y
/// A is an m x n matrix (row-major)
/// x is a vector of length n
/// y is a vector of length m
pub fn dgemv(m: usize, n: usize, alpha: f64, a: &[f64], x: &[f64], beta: f64, y: &mut [f64]) {
    assert_eq!(a.len(), m * n);
    assert_eq!(x.len(), n);
    assert_eq!(y.len(), m);

    // Scale y by beta
    if beta == 0.0 {
        y.fill(0.0);
    } else if beta != 1.0 {
        for yi in y.iter_mut() {
            *yi *= beta;
        }
    }

    // Compute alpha * A * x
    if alpha != 0.0 {
        for (i, row) in a.chunks_exact(n).enumerate() {
            y[i] += alpha * ddot(row, x);
        }
    }
}

/// Computes y = alpha * A^T * x + beta * y
/// A is an m x n matrix (row-major), but we compute A^T * x
/// x is a vector of length m
/// y is a vector of length n
pub fn dgemv_t(m: usize, n: usize, alpha: f64, a: &[f64], x: &[f64], beta: f64, y: &mut [f64]) {
    assert_eq!(a.len(), m * n);
    assert_eq!(x.len(), m);
    assert_eq!(y.len(), n);

    // Scale y by beta
    if beta == 0.0 {
        y.fill(0.0);
    } else if beta != 1.0 {
        for yi in y.iter_mut() {
            *yi *= beta;
        }
    }

    if alpha == 0.0 {
        return;
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            unsafe { dgemv_t_avx2_fma(m, n, alpha, a, x, y) };
            return;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            unsafe { dgemv_t_neon(m, n, alpha, a, x, y) };
            return;
        }
    }

    dgemv_t_scalar(m, n, alpha, a, x, y);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn dgemv_t_avx2_fma(m: usize, n: usize, alpha: f64, a: &[f64], x: &[f64], y: &mut [f64]) {
    const BLOCK_I: usize = 64;
    const BLOCK_J: usize = 32;

    let alpha_vec = _mm256_set1_pd(alpha);

    for jj in (0..n).step_by(BLOCK_J) {
        let j_max = std::cmp::min(jj + BLOCK_J, n);

        for ii in (0..m).step_by(BLOCK_I) {
            let i_max = std::cmp::min(ii + BLOCK_I, m);

            // Process 4 columns at a time with AVX2
            let mut j = jj;
            while j + 4 <= j_max && j + 4 <= n {
                let mut accum0 = _mm256_setzero_pd();
                let mut accum1 = _mm256_setzero_pd();
                let mut accum2 = _mm256_setzero_pd();
                let mut accum3 = _mm256_setzero_pd();

                // Process rows in chunks of 4 for better vectorization
                let mut i = ii;
                while i + 4 <= i_max {
                    let x_vec = _mm256_loadu_pd(x.as_ptr().add(i));

                    // Load 4 elements from 4 different columns for 4 rows
                    let k00 = *a.get_unchecked(i * n + j);
                    let k01 = *a.get_unchecked(i * n + j + 1);
                    let k02 = *a.get_unchecked(i * n + j + 2);
                    let k03 = *a.get_unchecked(i * n + j + 3);

                    let k10 = *a.get_unchecked((i + 1) * n + j);
                    let k11 = *a.get_unchecked((i + 1) * n + j + 1);
                    let k12 = *a.get_unchecked((i + 1) * n + j + 2);
                    let k13 = *a.get_unchecked((i + 1) * n + j + 3);

                    let k20 = *a.get_unchecked((i + 2) * n + j);
                    let k21 = *a.get_unchecked((i + 2) * n + j + 1);
                    let k22 = *a.get_unchecked((i + 2) * n + j + 2);
                    let k23 = *a.get_unchecked((i + 2) * n + j + 3);

                    let k30 = *a.get_unchecked((i + 3) * n + j);
                    let k31 = *a.get_unchecked((i + 3) * n + j + 1);
                    let k32 = *a.get_unchecked((i + 3) * n + j + 2);
                    let k33 = *a.get_unchecked((i + 3) * n + j + 3);

                    let k_col0 = _mm256_set_pd(k30, k20, k10, k00);
                    let k_col1 = _mm256_set_pd(k31, k21, k11, k01);
                    let k_col2 = _mm256_set_pd(k32, k22, k12, k02);
                    let k_col3 = _mm256_set_pd(k33, k23, k13, k03);

                    accum0 = _mm256_fmadd_pd(k_col0, x_vec, accum0);
                    accum1 = _mm256_fmadd_pd(k_col1, x_vec, accum1);
                    accum2 = _mm256_fmadd_pd(k_col2, x_vec, accum2);
                    accum3 = _mm256_fmadd_pd(k_col3, x_vec, accum3);

                    i += 4;
                }

                // Handle remaining rows
                while i < i_max {
                    let x_val = *x.get_unchecked(i);
                    let row_offset = i * n + j;

                    let x_vec = _mm256_set1_pd(x_val);
                    let k_vec = _mm256_loadu_pd(a.as_ptr().add(row_offset));
                    let prod = _mm256_mul_pd(k_vec, x_vec);

                    accum0 = _mm256_add_pd(accum0, _mm256_shuffle_pd(prod, prod, 0b0000));
                    accum1 = _mm256_add_pd(accum1, _mm256_shuffle_pd(prod, prod, 0b0101));
                    accum2 = _mm256_add_pd(accum2, _mm256_shuffle_pd(prod, prod, 0b1010));
                    accum3 = _mm256_add_pd(accum3, _mm256_shuffle_pd(prod, prod, 0b1111));

                    i += 1;
                }

                // Horizontal sum and store
                let sum0 = hsum_avx_pd(accum0);
                let sum1 = hsum_avx_pd(accum1);
                let sum2 = hsum_avx_pd(accum2);
                let sum3 = hsum_avx_pd(accum3);

                *y.get_unchecked_mut(j) += alpha * sum0;
                *y.get_unchecked_mut(j + 1) += alpha * sum1;
                *y.get_unchecked_mut(j + 2) += alpha * sum2;
                *y.get_unchecked_mut(j + 3) += alpha * sum3;

                j += 4;
            }

            // Clean up remaining columns
            while j < j_max {
                let mut sum = 0.0;
                for i in ii..i_max {
                    sum += *a.get_unchecked(i * n + j) * *x.get_unchecked(i);
                }
                *y.get_unchecked_mut(j) += alpha * sum;
                j += 1;
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn dgemv_t_neon(m: usize, n: usize, alpha: f64, a: &[f64], x: &[f64], y: &mut [f64]) {
    const BLOCK_I: usize = 32;
    const BLOCK_J: usize = 16;

    for jj in (0..n).step_by(BLOCK_J) {
        let j_max = std::cmp::min(jj + BLOCK_J, n);

        for ii in (0..m).step_by(BLOCK_I) {
            let i_max = std::cmp::min(ii + BLOCK_I, m);

            // Process 2 columns at a time with NEON
            let mut j = jj;
            while j + 2 <= j_max {
                let mut accum0 = vdupq_n_f64(0.0);
                let mut accum1 = vdupq_n_f64(0.0);

                let mut i = ii;
                while i + 2 <= i_max {
                    let x_vec = vld1q_f64(x.as_ptr().add(i));

                    // Gather matrix elements - THIS WAS THE BUG
                    // We need to properly accumulate column elements
                    let k00 = *a.get_unchecked(i * n + j);
                    let k10 = *a.get_unchecked((i + 1) * n + j);
                    let k_col0 = vsetq_lane_f64(k10, vsetq_lane_f64(k00, vdupq_n_f64(0.0), 0), 1);

                    let k01 = *a.get_unchecked(i * n + j + 1);
                    let k11 = *a.get_unchecked((i + 1) * n + j + 1);
                    let k_col1 = vsetq_lane_f64(k11, vsetq_lane_f64(k01, vdupq_n_f64(0.0), 0), 1);

                    accum0 = vfmaq_f64(accum0, k_col0, x_vec);
                    accum1 = vfmaq_f64(accum1, k_col1, x_vec);

                    i += 2;
                }

                // Handle remaining rows - FIX: just add single values, not vectors
                while i < i_max {
                    let x_val = *x.get_unchecked(i);
                    let k0 = *a.get_unchecked(i * n + j);
                    let k1 = *a.get_unchecked(i * n + j + 1);

                    // Just add the single product to lane 0
                    accum0 = vsetq_lane_f64(vgetq_lane_f64(accum0, 0) + k0 * x_val, accum0, 0);
                    accum1 = vsetq_lane_f64(vgetq_lane_f64(accum1, 0) + k1 * x_val, accum1, 0);

                    i += 1;
                }

                // Horizontal sum and store
                let sum0 = vgetq_lane_f64(accum0, 0) + vgetq_lane_f64(accum0, 1);
                let sum1 = vgetq_lane_f64(accum1, 0) + vgetq_lane_f64(accum1, 1);

                *y.get_unchecked_mut(j) += alpha * sum0;
                *y.get_unchecked_mut(j + 1) += alpha * sum1;

                j += 2;
            }

            // Clean up remaining columns
            while j < j_max {
                let mut sum = 0.0;
                for i in ii..i_max {
                    sum += *a.get_unchecked(i * n + j) * *x.get_unchecked(i);
                }
                *y.get_unchecked_mut(j) += alpha * sum;
                j += 1;
            }
        }
    }
}

fn dgemv_t_scalar(m: usize, n: usize, alpha: f64, a: &[f64], x: &[f64], y: &mut [f64]) {
    const BLOCK_I: usize = 64;
    const BLOCK_J: usize = 64;
    const UNROLL_J: usize = 4;

    for jj in (0..n).step_by(BLOCK_J) {
        for ii in (0..m).step_by(BLOCK_I) {
            let j_max = std::cmp::min(jj + BLOCK_J, n);
            let i_max = std::cmp::min(ii + BLOCK_I, m);

            let mut j = jj;
            while j + UNROLL_J <= j_max {
                let mut sum0 = 0.0;
                let mut sum1 = 0.0;
                let mut sum2 = 0.0;
                let mut sum3 = 0.0;

                for i in ii..i_max {
                    let x_val = x[i];
                    let base = i * n + j;
                    sum0 += a[base] * x_val;
                    sum1 += a[base + 1] * x_val;
                    sum2 += a[base + 2] * x_val;
                    sum3 += a[base + 3] * x_val;
                }

                y[j] += alpha * sum0;
                y[j + 1] += alpha * sum1;
                y[j + 2] += alpha * sum2;
                y[j + 3] += alpha * sum3;
                j += UNROLL_J;
            }

            while j < j_max {
                let mut sum = 0.0;
                for i in ii..i_max {
                    sum += a[i * n + j] * x[i];
                }
                y[j] += alpha * sum;
                j += 1;
            }
        }
    }
}

// ============= f32 Implementation =============

pub fn sgemv(m: usize, n: usize, alpha: f32, a: &[f32], x: &[f32], beta: f32, y: &mut [f32]) {
    assert_eq!(a.len(), m * n);
    assert_eq!(x.len(), n);
    assert_eq!(y.len(), m);

    if beta == 0.0 {
        y.fill(0.0);
    } else if beta != 1.0 {
        for yi in y.iter_mut() {
            *yi *= beta;
        }
    }

    if alpha != 0.0 {
        for (i, row) in a.chunks_exact(n).enumerate() {
            y[i] += alpha * sdot(row, x);
        }
    }
}

pub fn sgemv_t(m: usize, n: usize, alpha: f32, a: &[f32], x: &[f32], beta: f32, y: &mut [f32]) {
    assert_eq!(a.len(), m * n);
    assert_eq!(x.len(), m);
    assert_eq!(y.len(), n);

    if beta == 0.0 {
        y.fill(0.0);
    } else if beta != 1.0 {
        for yi in y.iter_mut() {
            *yi *= beta;
        }
    }

    if alpha == 0.0 {
        return;
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            unsafe { sgemv_t_avx2_fma(m, n, alpha, a, x, y) };
            return;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            unsafe { sgemv_t_neon(m, n, alpha, a, x, y) };
            return;
        }
    }

    sgemv_t_scalar(m, n, alpha, a, x, y);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn sgemv_t_avx2_fma(m: usize, n: usize, alpha: f32, a: &[f32], x: &[f32], y: &mut [f32]) {
    const BLOCK_I: usize = 128;
    const BLOCK_J: usize = 128;

    for jj in (0..n).step_by(BLOCK_J) {
        let j_max = std::cmp::min(jj + BLOCK_J, n);

        for ii in (0..m).step_by(BLOCK_I) {
            let i_max = std::cmp::min(ii + BLOCK_I, m);

            // Process 8 columns at a time
            let mut j = jj;
            while j + 8 <= j_max && j + 8 <= n {
                let mut accum0 = _mm256_setzero_ps();
                let mut accum1 = _mm256_setzero_ps();
                let mut accum2 = _mm256_setzero_ps();
                let mut accum3 = _mm256_setzero_ps();
                let mut accum4 = _mm256_setzero_ps();
                let mut accum5 = _mm256_setzero_ps();
                let mut accum6 = _mm256_setzero_ps();
                let mut accum7 = _mm256_setzero_ps();

                // Process rows in chunks of 8 for better vectorization
                let mut i = ii;
                while i + 8 <= i_max {
                    // Load 8 x values
                    let x_vec = _mm256_loadu_ps(x.as_ptr().add(i));

                    // For each of the 8 columns we're processing
                    // We need to gather the corresponding matrix elements
                    // Since matrix is row-major, elements for column j are at:
                    // a[i*n + j], a[(i+1)*n + j], ..., a[(i+7)*n + j]

                    // Column j+0
                    let k0 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j),
                        *a.get_unchecked((i + 6) * n + j),
                        *a.get_unchecked((i + 5) * n + j),
                        *a.get_unchecked((i + 4) * n + j),
                        *a.get_unchecked((i + 3) * n + j),
                        *a.get_unchecked((i + 2) * n + j),
                        *a.get_unchecked((i + 1) * n + j),
                        *a.get_unchecked(i * n + j),
                    );
                    accum0 = _mm256_fmadd_ps(k0, x_vec, accum0);

                    // Column j+1
                    let k1 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 1),
                        *a.get_unchecked((i + 6) * n + j + 1),
                        *a.get_unchecked((i + 5) * n + j + 1),
                        *a.get_unchecked((i + 4) * n + j + 1),
                        *a.get_unchecked((i + 3) * n + j + 1),
                        *a.get_unchecked((i + 2) * n + j + 1),
                        *a.get_unchecked((i + 1) * n + j + 1),
                        *a.get_unchecked(i * n + j + 1),
                    );
                    accum1 = _mm256_fmadd_ps(k1, x_vec, accum1);

                    // Column j+2
                    let k2 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 2),
                        *a.get_unchecked((i + 6) * n + j + 2),
                        *a.get_unchecked((i + 5) * n + j + 2),
                        *a.get_unchecked((i + 4) * n + j + 2),
                        *a.get_unchecked((i + 3) * n + j + 2),
                        *a.get_unchecked((i + 2) * n + j + 2),
                        *a.get_unchecked((i + 1) * n + j + 2),
                        *a.get_unchecked(i * n + j + 2),
                    );
                    accum2 = _mm256_fmadd_ps(k2, x_vec, accum2);

                    // Column j+3
                    let k3 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 3),
                        *a.get_unchecked((i + 6) * n + j + 3),
                        *a.get_unchecked((i + 5) * n + j + 3),
                        *a.get_unchecked((i + 4) * n + j + 3),
                        *a.get_unchecked((i + 3) * n + j + 3),
                        *a.get_unchecked((i + 2) * n + j + 3),
                        *a.get_unchecked((i + 1) * n + j + 3),
                        *a.get_unchecked(i * n + j + 3),
                    );
                    accum3 = _mm256_fmadd_ps(k3, x_vec, accum3);

                    // Column j+4
                    let k4 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 4),
                        *a.get_unchecked((i + 6) * n + j + 4),
                        *a.get_unchecked((i + 5) * n + j + 4),
                        *a.get_unchecked((i + 4) * n + j + 4),
                        *a.get_unchecked((i + 3) * n + j + 4),
                        *a.get_unchecked((i + 2) * n + j + 4),
                        *a.get_unchecked((i + 1) * n + j + 4),
                        *a.get_unchecked(i * n + j + 4),
                    );
                    accum4 = _mm256_fmadd_ps(k4, x_vec, accum4);

                    // Column j+5
                    let k5 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 5),
                        *a.get_unchecked((i + 6) * n + j + 5),
                        *a.get_unchecked((i + 5) * n + j + 5),
                        *a.get_unchecked((i + 4) * n + j + 5),
                        *a.get_unchecked((i + 3) * n + j + 5),
                        *a.get_unchecked((i + 2) * n + j + 5),
                        *a.get_unchecked((i + 1) * n + j + 5),
                        *a.get_unchecked(i * n + j + 5),
                    );
                    accum5 = _mm256_fmadd_ps(k5, x_vec, accum5);

                    // Column j+6
                    let k6 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 6),
                        *a.get_unchecked((i + 6) * n + j + 6),
                        *a.get_unchecked((i + 5) * n + j + 6),
                        *a.get_unchecked((i + 4) * n + j + 6),
                        *a.get_unchecked((i + 3) * n + j + 6),
                        *a.get_unchecked((i + 2) * n + j + 6),
                        *a.get_unchecked((i + 1) * n + j + 6),
                        *a.get_unchecked(i * n + j + 6),
                    );
                    accum6 = _mm256_fmadd_ps(k6, x_vec, accum6);

                    // Column j+7
                    let k7 = _mm256_set_ps(
                        *a.get_unchecked((i + 7) * n + j + 7),
                        *a.get_unchecked((i + 6) * n + j + 7),
                        *a.get_unchecked((i + 5) * n + j + 7),
                        *a.get_unchecked((i + 4) * n + j + 7),
                        *a.get_unchecked((i + 3) * n + j + 7),
                        *a.get_unchecked((i + 2) * n + j + 7),
                        *a.get_unchecked((i + 1) * n + j + 7),
                        *a.get_unchecked(i * n + j + 7),
                    );
                    accum7 = _mm256_fmadd_ps(k7, x_vec, accum7);

                    i += 8;
                }

                // Handle remaining rows (less than 8)
                while i < i_max {
                    let x_val = *x.get_unchecked(i);
                    let x_vec = _mm256_set1_ps(x_val);
                    let row_offset = i * n + j;

                    // Load 8 consecutive elements from row i, starting at column j
                    let k_vec = _mm256_loadu_ps(a.as_ptr().add(row_offset));

                    // Multiply and accumulate
                    // Each lane of k_vec corresponds to a different column
                    // We need to add each product to the corresponding accumulator
                    accum0 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset)),
                        _mm256_set1_ps(x_val),
                        accum0,
                    );
                    accum1 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 1)),
                        _mm256_set1_ps(x_val),
                        accum1,
                    );
                    accum2 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 2)),
                        _mm256_set1_ps(x_val),
                        accum2,
                    );
                    accum3 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 3)),
                        _mm256_set1_ps(x_val),
                        accum3,
                    );
                    accum4 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 4)),
                        _mm256_set1_ps(x_val),
                        accum4,
                    );
                    accum5 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 5)),
                        _mm256_set1_ps(x_val),
                        accum5,
                    );
                    accum6 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 6)),
                        _mm256_set1_ps(x_val),
                        accum6,
                    );
                    accum7 = _mm256_fmadd_ps(
                        _mm256_set1_ps(*a.get_unchecked(row_offset + 7)),
                        _mm256_set1_ps(x_val),
                        accum7,
                    );

                    i += 1;
                }

                // Horizontal sum and store results
                *y.get_unchecked_mut(j) += alpha * hsum_avx_ps(accum0);
                *y.get_unchecked_mut(j + 1) += alpha * hsum_avx_ps(accum1);
                *y.get_unchecked_mut(j + 2) += alpha * hsum_avx_ps(accum2);
                *y.get_unchecked_mut(j + 3) += alpha * hsum_avx_ps(accum3);
                *y.get_unchecked_mut(j + 4) += alpha * hsum_avx_ps(accum4);
                *y.get_unchecked_mut(j + 5) += alpha * hsum_avx_ps(accum5);
                *y.get_unchecked_mut(j + 6) += alpha * hsum_avx_ps(accum6);
                *y.get_unchecked_mut(j + 7) += alpha * hsum_avx_ps(accum7);

                j += 8;
            }

            // Clean up remaining columns
            while j < j_max {
                let mut sum = 0.0;
                for i in ii..i_max {
                    sum += *a.get_unchecked(i * n + j) * *x.get_unchecked(i);
                }
                *y.get_unchecked_mut(j) += alpha * sum;
                j += 1;
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn sgemv_t_neon(m: usize, n: usize, alpha: f32, a: &[f32], x: &[f32], y: &mut [f32]) {
    const BLOCK_I: usize = 64;
    const BLOCK_J: usize = 64;

    for jj in (0..n).step_by(BLOCK_J) {
        let j_max = std::cmp::min(jj + BLOCK_J, n);

        for ii in (0..m).step_by(BLOCK_I) {
            let i_max = std::cmp::min(ii + BLOCK_I, m);

            // Process 4 columns at a time with NEON
            let mut j = jj;
            while j + 4 <= j_max && j + 4 <= n {
                let mut sum0 = 0.0f32;
                let mut sum1 = 0.0f32;
                let mut sum2 = 0.0f32;
                let mut sum3 = 0.0f32;

                // Process rows in chunks of 4
                let mut i = ii;
                while i + 4 <= i_max {
                    // Load 4 x values
                    let x_vec = vld1q_f32(x.as_ptr().add(i));

                    // Gather matrix elements for each column
                    let k0 = vsetq_lane_f32(
                        *a.get_unchecked(i * n + j),
                        vsetq_lane_f32(
                            *a.get_unchecked((i + 1) * n + j),
                            vsetq_lane_f32(
                                *a.get_unchecked((i + 2) * n + j),
                                vsetq_lane_f32(
                                    *a.get_unchecked((i + 3) * n + j),
                                    vdupq_n_f32(0.0),
                                    3,
                                ),
                                2,
                            ),
                            1,
                        ),
                        0,
                    );

                    let k1 = vsetq_lane_f32(
                        *a.get_unchecked(i * n + j + 1),
                        vsetq_lane_f32(
                            *a.get_unchecked((i + 1) * n + j + 1),
                            vsetq_lane_f32(
                                *a.get_unchecked((i + 2) * n + j + 1),
                                vsetq_lane_f32(
                                    *a.get_unchecked((i + 3) * n + j + 1),
                                    vdupq_n_f32(0.0),
                                    3,
                                ),
                                2,
                            ),
                            1,
                        ),
                        0,
                    );

                    let k2 = vsetq_lane_f32(
                        *a.get_unchecked(i * n + j + 2),
                        vsetq_lane_f32(
                            *a.get_unchecked((i + 1) * n + j + 2),
                            vsetq_lane_f32(
                                *a.get_unchecked((i + 2) * n + j + 2),
                                vsetq_lane_f32(
                                    *a.get_unchecked((i + 3) * n + j + 2),
                                    vdupq_n_f32(0.0),
                                    3,
                                ),
                                2,
                            ),
                            1,
                        ),
                        0,
                    );

                    let k3 = vsetq_lane_f32(
                        *a.get_unchecked(i * n + j + 3),
                        vsetq_lane_f32(
                            *a.get_unchecked((i + 1) * n + j + 3),
                            vsetq_lane_f32(
                                *a.get_unchecked((i + 2) * n + j + 3),
                                vsetq_lane_f32(
                                    *a.get_unchecked((i + 3) * n + j + 3),
                                    vdupq_n_f32(0.0),
                                    3,
                                ),
                                2,
                            ),
                            1,
                        ),
                        0,
                    );

                    // Multiply and accumulate
                    sum0 += vaddvq_f32(vmulq_f32(k0, x_vec));
                    sum1 += vaddvq_f32(vmulq_f32(k1, x_vec));
                    sum2 += vaddvq_f32(vmulq_f32(k2, x_vec));
                    sum3 += vaddvq_f32(vmulq_f32(k3, x_vec));

                    i += 4;
                }

                // Handle remaining rows
                while i < i_max {
                    let x_val = *x.get_unchecked(i);
                    sum0 += *a.get_unchecked(i * n + j) * x_val;
                    sum1 += *a.get_unchecked(i * n + j + 1) * x_val;
                    sum2 += *a.get_unchecked(i * n + j + 2) * x_val;
                    sum3 += *a.get_unchecked(i * n + j + 3) * x_val;
                    i += 1;
                }

                // Store results
                *y.get_unchecked_mut(j) += alpha * sum0;
                *y.get_unchecked_mut(j + 1) += alpha * sum1;
                *y.get_unchecked_mut(j + 2) += alpha * sum2;
                *y.get_unchecked_mut(j + 3) += alpha * sum3;

                j += 4;
            }

            // Clean up remaining columns
            while j < j_max {
                let mut sum = 0.0;
                for i in ii..i_max {
                    sum += *a.get_unchecked(i * n + j) * *x.get_unchecked(i);
                }
                *y.get_unchecked_mut(j) += alpha * sum;
                j += 1;
            }
        }
    }
}

fn sgemv_t_scalar(m: usize, n: usize, alpha: f32, a: &[f32], x: &[f32], y: &mut [f32]) {
    const BLOCK_I: usize = 64;
    const BLOCK_J: usize = 64;
    const UNROLL_J: usize = 8;

    for jj in (0..n).step_by(BLOCK_J) {
        for ii in (0..m).step_by(BLOCK_I) {
            let j_max = std::cmp::min(jj + BLOCK_J, n);
            let i_max = std::cmp::min(ii + BLOCK_I, m);

            let mut j = jj;
            while j + UNROLL_J <= j_max {
                let mut sum = [0.0f32; UNROLL_J];

                for i in ii..i_max {
                    let x_val = x[i];
                    let base = i * n + j;
                    for k in 0..UNROLL_J {
                        sum[k] += a[base + k] * x_val;
                    }
                }

                for k in 0..UNROLL_J {
                    y[j + k] += alpha * sum[k];
                }
                j += UNROLL_J;
            }

            while j < j_max {
                let mut sum = 0.0;
                for i in ii..i_max {
                    sum += a[i * n + j] * x[i];
                }
                y[j] += alpha * sum;
                j += 1;
            }
        }
    }
}

// Helper functions
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn hsum_avx_pd(v: __m256d) -> f64 {
    let hi = _mm256_extractf128_pd(v, 1);
    let lo = _mm256_extractf128_pd(v, 0);
    let sum = _mm_add_pd(hi, lo);
    let hi64 = _mm_unpackhi_pd(sum, sum);
    _mm_cvtsd_f64(_mm_add_sd(sum, hi64))
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn hsum_avx_ps(v: __m256) -> f32 {
    let vlow = _mm256_extractf128_ps(v, 0);
    let vhigh = _mm256_extractf128_ps(v, 1);
    let sum128 = _mm_add_ps(vlow, vhigh);
    let sum64 = _mm_hadd_ps(sum128, sum128);
    let sum32 = _mm_hadd_ps(sum64, sum64);
    _mm_cvtss_f32(sum32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    // Helper function for creating test matrices
    fn create_test_matrix(m: usize, n: usize) -> Vec<f64> {
        let mut matrix = vec![0.0; m * n];
        for i in 0..m {
            for j in 0..n {
                matrix[i * n + j] = (i + 1) as f64 * 10.0 + (j + 1) as f64;
            }
        }
        matrix
    }

    fn create_test_matrix_f32(m: usize, n: usize) -> Vec<f32> {
        let mut matrix = vec![0.0; m * n];
        for i in 0..m {
            for j in 0..n {
                matrix[i * n + j] = (i + 1) as f32 * 10.0 + (j + 1) as f32;
            }
        }
        matrix
    }

    #[test]
    fn test_dgemv_basic() {
        // 3x3 matrix
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let x = vec![1.0, 2.0, 3.0];
        let mut y = vec![0.0; 3];

        dgemv(3, 3, 1.0, &a, &x, 0.0, &mut y);

        // Expected: [1*1+2*2+3*3, 4*1+5*2+6*3, 7*1+8*2+9*3] = [14, 32, 50]
        assert_abs_diff_eq!(y[0], 14.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[1], 32.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[2], 50.0, epsilon = 1e-10);
    }

    #[test]
    fn test_dgemv_alpha_beta() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let x = vec![1.0, 1.0];
        let mut y = vec![1.0, 1.0];

        // y = 2.0 * A * x + 3.0 * y
        dgemv(2, 2, 2.0, &a, &x, 3.0, &mut y);

        // A*x = [3, 7]
        // Expected: 2.0 * [3, 7] + 3.0 * [1, 1] = [6, 14] + [3, 3] = [9, 17]
        assert_abs_diff_eq!(y[0], 9.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[1], 17.0, epsilon = 1e-10);
    }

    #[test]
    fn test_dgemv_t_basic() {
        // 3x3 matrix
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let x = vec![1.0, 2.0, 3.0];
        let mut y = vec![0.0; 3];

        dgemv_t(3, 3, 1.0, &a, &x, 0.0, &mut y);

        // A^T * x means we sum down columns
        // Column 0: 1*1 + 4*2 + 7*3 = 1 + 8 + 21 = 30
        // Column 1: 2*1 + 5*2 + 8*3 = 2 + 10 + 24 = 36
        // Column 2: 3*1 + 6*2 + 9*3 = 3 + 12 + 27 = 42
        assert_abs_diff_eq!(y[0], 30.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[1], 36.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[2], 42.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sgemv_basic() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        let x = vec![1.0f32, 1.0, 1.0];
        let mut y = vec![0.0f32; 2];

        sgemv(2, 3, 1.0, &a, &x, 0.0, &mut y);

        // Expected: [6, 15]
        assert_abs_diff_eq!(y[0], 6.0, epsilon = 1e-6);
        assert_abs_diff_eq!(y[1], 15.0, epsilon = 1e-6);
    }

    #[test]
    fn test_sgemv_t_basic() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
        let x = vec![1.0f32, 2.0];
        let mut y = vec![0.0f32; 3];

        sgemv_t(2, 3, 1.0, &a, &x, 0.0, &mut y);

        // Column sums: [1*1+4*2, 2*1+5*2, 3*1+6*2] = [9, 12, 15]
        assert_abs_diff_eq!(y[0], 9.0, epsilon = 1e-6);
        assert_abs_diff_eq!(y[1], 12.0, epsilon = 1e-6);
        assert_abs_diff_eq!(y[2], 15.0, epsilon = 1e-6);
    }

    #[test]
    fn test_rectangular_matrices() {
        // Test 4x2 matrix
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let x = vec![1.0, 1.0];
        let mut y = vec![0.0; 4];

        dgemv(4, 2, 1.0, &a, &x, 0.0, &mut y);

        assert_abs_diff_eq!(y[0], 3.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[1], 7.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[2], 11.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[3], 15.0, epsilon = 1e-10);

        // Test transpose
        let x_t = vec![1.0, 1.0, 1.0, 1.0];
        let mut y_t = vec![0.0; 2];

        dgemv_t(4, 2, 1.0, &a, &x_t, 0.0, &mut y_t);

        // Sum of columns: [1+3+5+7, 2+4+6+8] = [16, 20]
        assert_abs_diff_eq!(y_t[0], 16.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y_t[1], 20.0, epsilon = 1e-10);
    }

    #[test]
    fn test_large_matrices() {
        // Test with various sizes to ensure blocking works correctly
        for size in [16, 32, 63, 64, 65, 128, 256].iter() {
            let m = *size;
            let n = *size;

            let a = create_test_matrix(m, n);
            let x = vec![1.0; n];
            let mut y = vec![0.0; m];

            dgemv(m, n, 1.0, &a, &x, 0.0, &mut y);

            // Each row sum should be: sum from j=1 to n of (10*i + j)
            // = 10*i*n + n*(n+1)/2
            for i in 0..m {
                let expected = 10.0 * (i + 1) as f64 * n as f64 + (n * (n + 1) / 2) as f64;
                assert_abs_diff_eq!(y[i], expected, epsilon = 1e-9);
            }
        }
    }

    #[test]
    fn test_transpose_consistency() {
        // Verify that gemv and gemv_t are consistent
        let m = 37; // Non-power-of-2 to test edge cases
        let n = 41;

        let a = create_test_matrix(m, n);

        // Test non-transposed
        let x1 = vec![1.0; n];
        let mut y1 = vec![0.0; m];
        dgemv(m, n, 2.5, &a, &x1, 0.0, &mut y1);

        // Test transposed
        let x2 = vec![1.0; m];
        let mut y2 = vec![0.0; n];
        dgemv_t(m, n, 2.5, &a, &x2, 0.0, &mut y2);

        // Manually compute expected values
        for i in 0..m {
            let mut sum = 0.0;
            for j in 0..n {
                sum += a[i * n + j];
            }
            assert_abs_diff_eq!(y1[i], 2.5 * sum, epsilon = 1e-9);
        }

        for j in 0..n {
            let mut sum = 0.0;
            for i in 0..m {
                sum += a[i * n + j];
            }
            assert_abs_diff_eq!(y2[j], 2.5 * sum, epsilon = 1e-9);
        }
    }

    #[test]
    fn test_zero_alpha() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let x = vec![1.0, 1.0];
        let mut y = vec![5.0, 6.0];

        // alpha = 0 should just scale y by beta
        dgemv(2, 2, 0.0, &a, &x, 2.0, &mut y);

        assert_abs_diff_eq!(y[0], 10.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[1], 12.0, epsilon = 1e-10);
    }

    #[test]
    fn test_zero_beta() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let x = vec![1.0, 1.0];
        let mut y = vec![999.0, 999.0]; // Should be overwritten

        // beta = 0 should ignore initial y
        dgemv(2, 2, 1.0, &a, &x, 0.0, &mut y);

        assert_abs_diff_eq!(y[0], 3.0, epsilon = 1e-10);
        assert_abs_diff_eq!(y[1], 7.0, epsilon = 1e-10);
    }

    #[test]
    fn test_f32_precision() {
        // Test that f32 gives reasonable precision
        let size = 100;
        let a = create_test_matrix_f32(size, size);
        let x = vec![1.0f32; size];
        let mut y = vec![0.0f32; size];

        sgemv(size, size, 1.0, &a, &x, 0.0, &mut y);

        // For row 0: sum of (10 + j) for j=1 to 100 = 10*100 + 5050 = 6050
        // For row 50: sum of (510 + j) for j=1 to 100 = 51000 + 5050 = 56050
        assert_abs_diff_eq!(y[0], 6050.0, epsilon = 1e-3);
        assert_abs_diff_eq!(y[50], 56050.0, epsilon = 1e-2);
    }

    #[test]
    #[should_panic]
    fn test_dimension_mismatch_panic() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let x = vec![1.0, 2.0, 3.0]; // Wrong size!
        let mut y = vec![0.0; 2];

        dgemv(2, 2, 1.0, &a, &x, 0.0, &mut y);
    }
}
