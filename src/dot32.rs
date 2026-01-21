#![allow(non_snake_case, clippy::missing_safety_doc)]

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
use std::arch::is_aarch64_feature_detected;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::x86_64::*;

pub fn sdot(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return unsafe { sdot_avx2_fma(a, b) };
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return unsafe { sdot_neon(a, b) };
        }
    }

    sdot_scalar(a, b)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,avx,fma")]
unsafe fn sdot_avx2_fma(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len();
    let mut accum0 = _mm256_setzero_ps();
    let mut accum1 = _mm256_setzero_ps();
    let mut accum2 = _mm256_setzero_ps();
    let mut accum3 = _mm256_setzero_ps();
    let mut accum4 = _mm256_setzero_ps();
    let mut accum5 = _mm256_setzero_ps();
    let mut accum6 = _mm256_setzero_ps();
    let mut accum7 = _mm256_setzero_ps();

    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let mut i = 0;

    // Process 128 elements per iteration (16 AVX registers × 8 floats)
    let chunk_size = 128;
    let iterations = len / chunk_size;
    let remainder_start = iterations * chunk_size;

    for _ in 0..iterations {
        let offset = i;

        // First 64 elements
        accum0 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset)),
            _mm256_loadu_ps(b_ptr.add(offset)),
            accum0,
        );
        accum1 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 8)),
            _mm256_loadu_ps(b_ptr.add(offset + 8)),
            accum1,
        );
        accum2 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 16)),
            _mm256_loadu_ps(b_ptr.add(offset + 16)),
            accum2,
        );
        accum3 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 24)),
            _mm256_loadu_ps(b_ptr.add(offset + 24)),
            accum3,
        );
        accum4 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 32)),
            _mm256_loadu_ps(b_ptr.add(offset + 32)),
            accum4,
        );
        accum5 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 40)),
            _mm256_loadu_ps(b_ptr.add(offset + 40)),
            accum5,
        );
        accum6 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 48)),
            _mm256_loadu_ps(b_ptr.add(offset + 48)),
            accum6,
        );
        accum7 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 56)),
            _mm256_loadu_ps(b_ptr.add(offset + 56)),
            accum7,
        );

        // Second 64 elements
        accum0 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 64)),
            _mm256_loadu_ps(b_ptr.add(offset + 64)),
            accum0,
        );
        accum1 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 72)),
            _mm256_loadu_ps(b_ptr.add(offset + 72)),
            accum1,
        );
        accum2 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 80)),
            _mm256_loadu_ps(b_ptr.add(offset + 80)),
            accum2,
        );
        accum3 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 88)),
            _mm256_loadu_ps(b_ptr.add(offset + 88)),
            accum3,
        );
        accum4 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 96)),
            _mm256_loadu_ps(b_ptr.add(offset + 96)),
            accum4,
        );
        accum5 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 104)),
            _mm256_loadu_ps(b_ptr.add(offset + 104)),
            accum5,
        );
        accum6 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 112)),
            _mm256_loadu_ps(b_ptr.add(offset + 112)),
            accum6,
        );
        accum7 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 120)),
            _mm256_loadu_ps(b_ptr.add(offset + 120)),
            accum7,
        );

        i += chunk_size;
    }

    // Combine accumulators
    accum0 = _mm256_add_ps(accum0, accum1);
    accum2 = _mm256_add_ps(accum2, accum3);
    accum4 = _mm256_add_ps(accum4, accum5);
    accum6 = _mm256_add_ps(accum6, accum7);

    accum0 = _mm256_add_ps(accum0, accum2);
    accum4 = _mm256_add_ps(accum4, accum6);

    let total = _mm256_add_ps(accum0, accum4);

    // Horizontal sum
    let sum = hsum_avx(total);

    // Process remaining elements with optimized scalar
    sum + sdot_scalar(&a[remainder_start..], &b[remainder_start..])
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn sdot_neon(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len();
    let mut accum0 = vdupq_n_f32(0.0);
    let mut accum1 = vdupq_n_f32(0.0);
    let mut accum2 = vdupq_n_f32(0.0);
    let mut accum3 = vdupq_n_f32(0.0);
    let mut accum4 = vdupq_n_f32(0.0);
    let mut accum5 = vdupq_n_f32(0.0);
    let mut accum6 = vdupq_n_f32(0.0);
    let mut accum7 = vdupq_n_f32(0.0);

    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let mut i = 0;

    // Process 64 elements per iteration (8 NEON registers × 8 floats)
    let chunk_size = 64;
    let iterations = len / chunk_size;
    let remainder_start = iterations * chunk_size;

    for _ in 0..iterations {
        let offset = i;

        // Load and process 8 vectors
        accum0 = vfmaq_f32(
            accum0,
            vld1q_f32(a_ptr.add(offset)),
            vld1q_f32(b_ptr.add(offset)),
        );
        accum1 = vfmaq_f32(
            accum1,
            vld1q_f32(a_ptr.add(offset + 4)),
            vld1q_f32(b_ptr.add(offset + 4)),
        );
        accum2 = vfmaq_f32(
            accum2,
            vld1q_f32(a_ptr.add(offset + 8)),
            vld1q_f32(b_ptr.add(offset + 8)),
        );
        accum3 = vfmaq_f32(
            accum3,
            vld1q_f32(a_ptr.add(offset + 12)),
            vld1q_f32(b_ptr.add(offset + 12)),
        );
        accum4 = vfmaq_f32(
            accum4,
            vld1q_f32(a_ptr.add(offset + 16)),
            vld1q_f32(b_ptr.add(offset + 16)),
        );
        accum5 = vfmaq_f32(
            accum5,
            vld1q_f32(a_ptr.add(offset + 20)),
            vld1q_f32(b_ptr.add(offset + 20)),
        );
        accum6 = vfmaq_f32(
            accum6,
            vld1q_f32(a_ptr.add(offset + 24)),
            vld1q_f32(b_ptr.add(offset + 24)),
        );
        accum7 = vfmaq_f32(
            accum7,
            vld1q_f32(a_ptr.add(offset + 28)),
            vld1q_f32(b_ptr.add(offset + 28)),
        );

        // Second set of 32 elements
        accum0 = vfmaq_f32(
            accum0,
            vld1q_f32(a_ptr.add(offset + 32)),
            vld1q_f32(b_ptr.add(offset + 32)),
        );
        accum1 = vfmaq_f32(
            accum1,
            vld1q_f32(a_ptr.add(offset + 36)),
            vld1q_f32(b_ptr.add(offset + 36)),
        );
        accum2 = vfmaq_f32(
            accum2,
            vld1q_f32(a_ptr.add(offset + 40)),
            vld1q_f32(b_ptr.add(offset + 40)),
        );
        accum3 = vfmaq_f32(
            accum3,
            vld1q_f32(a_ptr.add(offset + 44)),
            vld1q_f32(b_ptr.add(offset + 44)),
        );
        accum4 = vfmaq_f32(
            accum4,
            vld1q_f32(a_ptr.add(offset + 48)),
            vld1q_f32(b_ptr.add(offset + 48)),
        );
        accum5 = vfmaq_f32(
            accum5,
            vld1q_f32(a_ptr.add(offset + 52)),
            vld1q_f32(b_ptr.add(offset + 52)),
        );
        accum6 = vfmaq_f32(
            accum6,
            vld1q_f32(a_ptr.add(offset + 56)),
            vld1q_f32(b_ptr.add(offset + 56)),
        );
        accum7 = vfmaq_f32(
            accum7,
            vld1q_f32(a_ptr.add(offset + 60)),
            vld1q_f32(b_ptr.add(offset + 60)),
        );

        i += chunk_size;
    }

    // Combine accumulators
    accum0 = vaddq_f32(accum0, accum1);
    accum2 = vaddq_f32(accum2, accum3);
    accum4 = vaddq_f32(accum4, accum5);
    accum6 = vaddq_f32(accum6, accum7);

    accum0 = vaddq_f32(accum0, accum2);
    accum4 = vaddq_f32(accum4, accum6);

    let total = vaddq_f32(accum0, accum4);

    // Horizontal sum
    let sum = hsum_neon(total);

    // Process remaining elements
    sum + sdot_scalar(&a[remainder_start..], &b[remainder_start..])
}

#[inline(always)]
fn sdot_scalar(a: &[f32], b: &[f32]) -> f32 {
    let mut sum0 = 0.0;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;
    let mut sum4 = 0.0;
    let mut sum5 = 0.0;
    let mut sum6 = 0.0;
    let mut sum7 = 0.0;

    let mut i = 0;
    let len = a.len();
    let upper = len - (len % 8);

    while i < upper {
        sum0 += a[i] * b[i];
        sum1 += a[i + 1] * b[i + 1];
        sum2 += a[i + 2] * b[i + 2];
        sum3 += a[i + 3] * b[i + 3];
        sum4 += a[i + 4] * b[i + 4];
        sum5 += a[i + 5] * b[i + 5];
        sum6 += a[i + 6] * b[i + 6];
        sum7 += a[i + 7] * b[i + 7];
        i += 8;
    }

    let mut total = sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7;

    // Process remaining elements
    while i < len {
        total += a[i] * b[i];
        i += 1;
    }

    total
}

// --- Squared Euclidean Distance for f32 ---

/// Computes the squared Euclidean distance for f32 slices: sum((a[i] - b[i])^2)
/// # Panics
/// Panics if the slices have different lengths
pub fn sl2sq(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(
        a.len(),
        b.len(),
        "Slices must have equal length for squared euclidean distance"
    );
    if a.is_empty() {
        return 0.0;
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return unsafe { sl2sq_avx2_fma(a, b) };
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            return unsafe { sl2sq_neon(a, b) };
        }
    }
    sl2sq_scalar(a, b)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn sl2sq_avx2_fma(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len();
    let mut accum0 = _mm256_setzero_ps();
    let mut accum1 = _mm256_setzero_ps();
    let mut accum2 = _mm256_setzero_ps();
    let mut accum3 = _mm256_setzero_ps();
    let mut accum4 = _mm256_setzero_ps();
    let mut accum5 = _mm256_setzero_ps();
    let mut accum6 = _mm256_setzero_ps();
    let mut accum7 = _mm256_setzero_ps();

    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let mut i = 0;

    let chunk_size = 128; // 8 accums * 2 vectors/accum * 8 floats/vector
    let iterations = len / chunk_size;
    let remainder_start = iterations * chunk_size;

    for _ in 0..iterations {
        let offset = i;
        let mut diff: __m256;

        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset)),
            _mm256_loadu_ps(b_ptr.add(offset)),
        );
        accum0 = _mm256_fmadd_ps(diff, diff, accum0);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 8)),
            _mm256_loadu_ps(b_ptr.add(offset + 8)),
        );
        accum1 = _mm256_fmadd_ps(diff, diff, accum1);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 16)),
            _mm256_loadu_ps(b_ptr.add(offset + 16)),
        );
        accum2 = _mm256_fmadd_ps(diff, diff, accum2);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 24)),
            _mm256_loadu_ps(b_ptr.add(offset + 24)),
        );
        accum3 = _mm256_fmadd_ps(diff, diff, accum3);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 32)),
            _mm256_loadu_ps(b_ptr.add(offset + 32)),
        );
        accum4 = _mm256_fmadd_ps(diff, diff, accum4);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 40)),
            _mm256_loadu_ps(b_ptr.add(offset + 40)),
        );
        accum5 = _mm256_fmadd_ps(diff, diff, accum5);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 48)),
            _mm256_loadu_ps(b_ptr.add(offset + 48)),
        );
        accum6 = _mm256_fmadd_ps(diff, diff, accum6);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 56)),
            _mm256_loadu_ps(b_ptr.add(offset + 56)),
        );
        accum7 = _mm256_fmadd_ps(diff, diff, accum7);

        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 64)),
            _mm256_loadu_ps(b_ptr.add(offset + 64)),
        );
        accum0 = _mm256_fmadd_ps(diff, diff, accum0);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 72)),
            _mm256_loadu_ps(b_ptr.add(offset + 72)),
        );
        accum1 = _mm256_fmadd_ps(diff, diff, accum1);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 80)),
            _mm256_loadu_ps(b_ptr.add(offset + 80)),
        );
        accum2 = _mm256_fmadd_ps(diff, diff, accum2);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 88)),
            _mm256_loadu_ps(b_ptr.add(offset + 88)),
        );
        accum3 = _mm256_fmadd_ps(diff, diff, accum3);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 96)),
            _mm256_loadu_ps(b_ptr.add(offset + 96)),
        );
        accum4 = _mm256_fmadd_ps(diff, diff, accum4);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 104)),
            _mm256_loadu_ps(b_ptr.add(offset + 104)),
        );
        accum5 = _mm256_fmadd_ps(diff, diff, accum5);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 112)),
            _mm256_loadu_ps(b_ptr.add(offset + 112)),
        );
        accum6 = _mm256_fmadd_ps(diff, diff, accum6);
        diff = _mm256_sub_ps(
            _mm256_loadu_ps(a_ptr.add(offset + 120)),
            _mm256_loadu_ps(b_ptr.add(offset + 120)),
        );
        accum7 = _mm256_fmadd_ps(diff, diff, accum7);
        i += chunk_size;
    }

    accum0 = _mm256_add_ps(accum0, accum1);
    accum2 = _mm256_add_ps(accum2, accum3);
    accum4 = _mm256_add_ps(accum4, accum5);
    accum6 = _mm256_add_ps(accum6, accum7);
    accum0 = _mm256_add_ps(accum0, accum2);
    accum4 = _mm256_add_ps(accum4, accum6);
    let total_vec = _mm256_add_ps(accum0, accum4);
    let sum = hsum_avx(total_vec);

    sum + sl2sq_scalar(&a[remainder_start..], &b[remainder_start..])
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn sl2sq_neon(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len();
    let mut accum0 = vdupq_n_f32(0.0);
    let mut accum1 = vdupq_n_f32(0.0);
    let mut accum2 = vdupq_n_f32(0.0);
    let mut accum3 = vdupq_n_f32(0.0);
    let mut accum4 = vdupq_n_f32(0.0);
    let mut accum5 = vdupq_n_f32(0.0);
    let mut accum6 = vdupq_n_f32(0.0);
    let mut accum7 = vdupq_n_f32(0.0);

    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let mut i = 0;

    let chunk_size = 64; // 8 accums * 2 vectors/accum * 4 floats/vector
    let iterations = len / chunk_size;
    let remainder_start = iterations * chunk_size;

    for _ in 0..iterations {
        let offset = i;
        let mut diff: float32x4_t;

        diff = vsubq_f32(vld1q_f32(a_ptr.add(offset)), vld1q_f32(b_ptr.add(offset)));
        accum0 = vfmaq_f32(accum0, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 4)),
            vld1q_f32(b_ptr.add(offset + 4)),
        );
        accum1 = vfmaq_f32(accum1, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 8)),
            vld1q_f32(b_ptr.add(offset + 8)),
        );
        accum2 = vfmaq_f32(accum2, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 12)),
            vld1q_f32(b_ptr.add(offset + 12)),
        );
        accum3 = vfmaq_f32(accum3, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 16)),
            vld1q_f32(b_ptr.add(offset + 16)),
        );
        accum4 = vfmaq_f32(accum4, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 20)),
            vld1q_f32(b_ptr.add(offset + 20)),
        );
        accum5 = vfmaq_f32(accum5, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 24)),
            vld1q_f32(b_ptr.add(offset + 24)),
        );
        accum6 = vfmaq_f32(accum6, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 28)),
            vld1q_f32(b_ptr.add(offset + 28)),
        );
        accum7 = vfmaq_f32(accum7, diff, diff);

        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 32)),
            vld1q_f32(b_ptr.add(offset + 32)),
        );
        accum0 = vfmaq_f32(accum0, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 36)),
            vld1q_f32(b_ptr.add(offset + 36)),
        );
        accum1 = vfmaq_f32(accum1, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 40)),
            vld1q_f32(b_ptr.add(offset + 40)),
        );
        accum2 = vfmaq_f32(accum2, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 44)),
            vld1q_f32(b_ptr.add(offset + 44)),
        );
        accum3 = vfmaq_f32(accum3, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 48)),
            vld1q_f32(b_ptr.add(offset + 48)),
        );
        accum4 = vfmaq_f32(accum4, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 52)),
            vld1q_f32(b_ptr.add(offset + 52)),
        );
        accum5 = vfmaq_f32(accum5, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 56)),
            vld1q_f32(b_ptr.add(offset + 56)),
        );
        accum6 = vfmaq_f32(accum6, diff, diff);
        diff = vsubq_f32(
            vld1q_f32(a_ptr.add(offset + 60)),
            vld1q_f32(b_ptr.add(offset + 60)),
        );
        accum7 = vfmaq_f32(accum7, diff, diff);
        i += chunk_size;
    }

    accum0 = vaddq_f32(accum0, accum1);
    accum2 = vaddq_f32(accum2, accum3);
    accum4 = vaddq_f32(accum4, accum5);
    accum6 = vaddq_f32(accum6, accum7);
    accum0 = vaddq_f32(accum0, accum2);
    accum4 = vaddq_f32(accum4, accum6);
    let total_vec = vaddq_f32(accum0, accum4);
    let sum = hsum_neon(total_vec);

    sum + sl2sq_scalar(&a[remainder_start..], &b[remainder_start..])
}

#[inline(always)]
fn sl2sq_scalar(a: &[f32], b: &[f32]) -> f32 {
    let len = a.len();
    if len == 0 {
        return 0.0;
    }
    let mut sum0 = 0.0;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;
    let mut sum4 = 0.0;
    let mut sum5 = 0.0;
    let mut sum6 = 0.0;
    let mut sum7 = 0.0;
    let mut i = 0;
    let upper = len - (len % 8); // Unroll 8 times
    while i < upper {
        let d0 = a[i] - b[i];
        sum0 += d0 * d0;
        let d1 = a[i + 1] - b[i + 1];
        sum1 += d1 * d1;
        let d2 = a[i + 2] - b[i + 2];
        sum2 += d2 * d2;
        let d3 = a[i + 3] - b[i + 3];
        sum3 += d3 * d3;
        let d4 = a[i + 4] - b[i + 4];
        sum4 += d4 * d4;
        let d5 = a[i + 5] - b[i + 5];
        sum5 += d5 * d5;
        let d6 = a[i + 6] - b[i + 6];
        sum6 += d6 * d6;
        let d7 = a[i + 7] - b[i + 7];
        sum7 += d7 * d7;
        i += 8;
    }
    let mut total_sum = sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7;
    while i < len {
        let d = a[i] - b[i];
        total_sum += d * d;
        i += 1;
    }
    total_sum
}

// --- Vector Normalization ---

/// Normalizes a vector in-place to unit length.
/// Returns the original norm (L2 norm) of the vector.
/// If the vector has zero norm, it remains unchanged and 0.0 is returned.
pub fn snormalize(v: &mut [f32]) -> f32 {
    if v.is_empty() {
        return 0.0;
    }

    let norm_sq = sdot(v, v);
    if norm_sq == 0.0 {
        return 0.0;
    }

    let norm = norm_sq.sqrt();
    let inv_norm = 1.0 / norm;

    sscale(v, inv_norm);

    norm
}

/// Scales a vector in-place by a scalar: v[i] *= scale
pub fn sscale(v: &mut [f32], scale: f32) {
    if v.is_empty() {
        return;
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe { sscale_avx2(v, scale) };
            return;
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        if is_aarch64_feature_detected!("neon") {
            unsafe { sscale_neon(v, scale) };
            return;
        }
    }

    sscale_scalar(v, scale);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn sscale_avx2(v: &mut [f32], scale: f32) {
    let len = v.len();
    let scale_vec = _mm256_set1_ps(scale);
    let v_ptr = v.as_mut_ptr();
    let mut i = 0;

    // Process 32 elements per iteration (4 vectors × 8 elements)
    let chunk_size = 32;
    let iterations = len / chunk_size;
    let remainder_start = iterations * chunk_size;

    for _ in 0..iterations {
        let offset = i;

        let v0 = _mm256_loadu_ps(v_ptr.add(offset));
        _mm256_storeu_ps(v_ptr.add(offset), _mm256_mul_ps(v0, scale_vec));

        let v1 = _mm256_loadu_ps(v_ptr.add(offset + 8));
        _mm256_storeu_ps(v_ptr.add(offset + 8), _mm256_mul_ps(v1, scale_vec));

        let v2 = _mm256_loadu_ps(v_ptr.add(offset + 16));
        _mm256_storeu_ps(v_ptr.add(offset + 16), _mm256_mul_ps(v2, scale_vec));

        let v3 = _mm256_loadu_ps(v_ptr.add(offset + 24));
        _mm256_storeu_ps(v_ptr.add(offset + 24), _mm256_mul_ps(v3, scale_vec));

        i += chunk_size;
    }

    // Process remaining elements
    sscale_scalar(&mut v[remainder_start..], scale);
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn sscale_neon(v: &mut [f32], scale: f32) {
    let len = v.len();
    let scale_vec = vdupq_n_f32(scale);
    let v_ptr = v.as_mut_ptr();
    let mut i = 0;

    // Process 16 elements per iteration (4 vectors × 4 elements)
    let chunk_size = 16;
    let iterations = len / chunk_size;
    let remainder_start = iterations * chunk_size;

    for _ in 0..iterations {
        let offset = i;

        let v0 = vld1q_f32(v_ptr.add(offset));
        vst1q_f32(v_ptr.add(offset), vmulq_f32(v0, scale_vec));

        let v1 = vld1q_f32(v_ptr.add(offset + 4));
        vst1q_f32(v_ptr.add(offset + 4), vmulq_f32(v1, scale_vec));

        let v2 = vld1q_f32(v_ptr.add(offset + 8));
        vst1q_f32(v_ptr.add(offset + 8), vmulq_f32(v2, scale_vec));

        let v3 = vld1q_f32(v_ptr.add(offset + 12));
        vst1q_f32(v_ptr.add(offset + 12), vmulq_f32(v3, scale_vec));

        i += chunk_size;
    }

    // Process remaining elements
    sscale_scalar(&mut v[remainder_start..], scale);
}

#[inline(always)]
fn sscale_scalar(v: &mut [f32], scale: f32) {
    let len = v.len();
    let mut i = 0;
    let upper = len - (len % 8);

    while i < upper {
        v[i] *= scale;
        v[i + 1] *= scale;
        v[i + 2] *= scale;
        v[i + 3] *= scale;
        v[i + 4] *= scale;
        v[i + 5] *= scale;
        v[i + 6] *= scale;
        v[i + 7] *= scale;
        i += 8;
    }

    while i < len {
        v[i] *= scale;
        i += 1;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn hsum_avx(v: __m256) -> f32 {
    let vlow = _mm256_extractf128_ps(v, 0);
    let vhigh = _mm256_extractf128_ps(v, 1);
    let sum128 = _mm_add_ps(vlow, vhigh);
    let sum64 = _mm_hadd_ps(sum128, sum128);
    let sum32 = _mm_hadd_ps(sum64, sum64);
    _mm_cvtss_f32(sum32)
}

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn hsum_neon(v: float32x4_t) -> f32 {
    let sum_pair = vadd_f32(vget_low_f32(v), vget_high_f32(v));
    let sum = vpadd_f32(sum_pair, sum_pair);
    vget_lane_f32(sum, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rand::{thread_rng, Rng};

    fn random_f32_vec(len: usize) -> Vec<f32> {
        let mut rng = thread_rng();
        (0..len).map(|_| rng.gen_range(-1.0..1.0)).collect()
    }

    #[test]
    fn test_dot_consistency_small() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_abs_diff_eq!(sdot(&a, &b), 32.0, epsilon = 1e-10);
    }

    #[test]
    fn test_dot_spcialized_agains_general() {
        let a = random_f32_vec(1024);
        let b = random_f32_vec(1024);

        let simd_result = sdot(&a, &b);
        let scalar_result = sdot_scalar(&a, &b);

        assert_abs_diff_eq!(simd_result, scalar_result, epsilon = 1e-5);
    }

    #[test]
    fn test_edge_cases() {
        // Zeros
        let zeros = vec![0.0; 100];
        assert_abs_diff_eq!(sdot(&zeros, &zeros), 0.0, epsilon = 1e-10);

        // Denormals
        let denormals = vec![f32::MIN_POSITIVE; 100];
        let expected = (f32::MIN_POSITIVE * f32::MIN_POSITIVE) * 100.0;
        assert_abs_diff_eq!(sdot(&denormals, &denormals), expected, epsilon = 1e-10);

        // Mixed signs
        let a = vec![1.0, -1.0, 2.0, -2.0];
        let b = vec![1.0, 1.0, 1.0, 1.0];
        assert_abs_diff_eq!(sdot(&a, &b), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_various_lengths() {
        for len in 0..50 {
            let a = random_f32_vec(len);
            let b = random_f32_vec(len);

            let simd_result = sdot(&a, &b);
            let scalar_result = sdot_scalar(&a, &b);

            assert_abs_diff_eq!(simd_result, scalar_result, epsilon = 1e-6);
        }
    }

    // --- Squared Euclidean Distance Tests ---
    #[test]
    fn test_l2sq_consistency_small() {
        let a = vec![1.0f32, 2.0, 3.0];
        let b = vec![4.0f32, 5.0, 6.0];
        // (1-4)^2 + (2-5)^2 + (3-6)^2 = 9+9+9 = 27
        assert_abs_diff_eq!(sl2sq(&a, &b), 27.0, epsilon = 1e-6);
    }

    #[test]
    fn test_l2sq_zero_distance() {
        let a = vec![1.0f32, -2.5, 3.14, 0.0];
        assert_abs_diff_eq!(sl2sq(&a, &a), 0.0, epsilon = 1e-7);
        let empty: Vec<f32> = vec![];
        assert_abs_diff_eq!(sl2sq(&empty, &empty), 0.0, epsilon = 1e-7);
    }

    #[test]
    fn test_l2sq_specialized_vs_general() {
        for len in [
            0, 1, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 129, 1000, 1024,
        ] {
            let a = random_f32_vec(len);
            let b = random_f32_vec(len);

            let simd_result = sl2sq(&a, &b);
            let scalar_result = sl2sq_scalar(&a, &b);

            // Use relative comparison for large values - f32 has ~7 digits of precision
            let rel_eps = (simd_result.abs() + scalar_result.abs()) * 1e-5 + 1e-4;
            assert!(
                (simd_result - scalar_result).abs() < rel_eps,
                "SIMD and scalar l2sq differ by {} (threshold {})",
                (simd_result - scalar_result).abs(),
                rel_eps
            );
        }
    }

    // --- Normalization Tests ---
    #[test]
    fn test_normalize_basic() {
        let mut v = vec![3.0f32, 4.0];
        let norm = snormalize(&mut v);
        assert_abs_diff_eq!(norm, 5.0, epsilon = 1e-6);
        assert_abs_diff_eq!(v[0], 0.6, epsilon = 1e-6);
        assert_abs_diff_eq!(v[1], 0.8, epsilon = 1e-6);
    }

    #[test]
    fn test_normalize_unit_length() {
        let mut v = random_f32_vec(100);
        snormalize(&mut v);

        // Check that the result has unit length
        let norm_sq = sdot(&v, &v);
        assert_abs_diff_eq!(norm_sq, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let mut v = vec![0.0f32, 0.0, 0.0];
        let norm = snormalize(&mut v);
        assert_abs_diff_eq!(norm, 0.0, epsilon = 1e-10);
        // Vector should remain unchanged
        assert_abs_diff_eq!(v[0], 0.0, epsilon = 1e-10);
        assert_abs_diff_eq!(v[1], 0.0, epsilon = 1e-10);
        assert_abs_diff_eq!(v[2], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_scale_basic() {
        let mut v = vec![1.0f32, 2.0, 3.0, 4.0];
        sscale(&mut v, 2.0);
        assert_abs_diff_eq!(v[0], 2.0, epsilon = 1e-6);
        assert_abs_diff_eq!(v[1], 4.0, epsilon = 1e-6);
        assert_abs_diff_eq!(v[2], 6.0, epsilon = 1e-6);
        assert_abs_diff_eq!(v[3], 8.0, epsilon = 1e-6);
    }

    #[test]
    fn test_scale_various_lengths() {
        for len in [0, 1, 3, 7, 15, 16, 17, 31, 32, 33, 100] {
            let mut v = random_f32_vec(len);
            let original = v.clone();
            let scale = 2.5;

            sscale(&mut v, scale);

            for i in 0..len {
                assert_abs_diff_eq!(v[i], original[i] * scale, epsilon = 1e-5);
            }
        }
    }
}
