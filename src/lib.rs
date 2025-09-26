mod dot32;
mod dot64;
mod gemm;
mod gemv;
// pub mod quantized;
use num_traits::{One, Zero};

pub trait Blas: Sized + Copy + Zero + One {
    fn dot(a: &[Self], b: &[Self]) -> Self;
    fn l2(a: &[Self], b: &[Self]) -> Self;
    fn gemv(m: usize, n: usize, alpha: Self, a: &[Self], x: &[Self], beta: Self, y: &mut [Self]);
    fn gemv_t(m: usize, n: usize, alpha: Self, a: &[Self], x: &[Self], beta: Self, y: &mut [Self]);

    // New GEMM methods
    fn gemm(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    );

    fn gemm_tn(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    );

    fn gemm_nt(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    );

    fn gemm_tt(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    );
}

impl Blas for f32 {
    #[inline]
    fn dot(a: &[Self], b: &[Self]) -> Self {
        dot32::sdot(a, b)
    }

    #[inline]
    fn l2(a: &[Self], b: &[Self]) -> Self {
        dot32::sl2(a, b)
    }

    #[inline]
    fn gemv(m: usize, n: usize, alpha: Self, a: &[Self], x: &[Self], beta: Self, y: &mut [Self]) {
        gemv::sgemv(m, n, alpha, a, x, beta, y)
    }

    #[inline]
    fn gemv_t(m: usize, n: usize, alpha: Self, a: &[Self], x: &[Self], beta: Self, y: &mut [Self]) {
        gemv::sgemv_t(m, n, alpha, a, x, beta, y)
    }

    #[inline]
    fn gemm(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::sgemm(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[inline]
    fn gemm_tn(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::sgemm_tn(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[inline]
    fn gemm_nt(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::sgemm_nt(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[inline]
    fn gemm_tt(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::sgemm_tt(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }
}

impl Blas for f64 {
    #[inline]
    fn dot(a: &[Self], b: &[Self]) -> Self {
        dot64::ddot(a, b)
    }

    #[inline]
    fn l2(a: &[Self], b: &[Self]) -> Self {
        dot64::dl2(a, b)
    }

    #[inline]
    fn gemv(m: usize, n: usize, alpha: Self, a: &[Self], x: &[Self], beta: Self, y: &mut [Self]) {
        gemv::dgemv(m, n, alpha, a, x, beta, y)
    }

    #[inline]
    fn gemv_t(m: usize, n: usize, alpha: Self, a: &[Self], x: &[Self], beta: Self, y: &mut [Self]) {
        gemv::dgemv_t(m, n, alpha, a, x, beta, y)
    }

    #[inline]
    fn gemm(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::dgemm(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[inline]
    fn gemm_tn(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::dgemm_tn(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[inline]
    fn gemm_nt(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::dgemm_nt(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[inline]
    fn gemm_tt(
        m: usize,
        n: usize,
        k: usize,
        alpha: Self,
        a: &[Self],
        lda: usize,
        b: &[Self],
        ldb: usize,
        beta: Self,
        c: &mut [Self],
        ldc: usize,
    ) {
        gemm::dgemm_tt(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
    }
}

// Public API - these are all users need
#[inline]
pub fn dot<T: Blas>(a: &[T], b: &[T]) -> T {
    T::dot(a, b)
}

#[inline]
pub fn l2<T: Blas>(a: &[T], b: &[T]) -> T {
    T::l2(a, b)
}

#[inline]
pub fn gemv<T: Blas>(m: usize, n: usize, alpha: T, a: &[T], x: &[T], beta: T, y: &mut [T]) {
    T::gemv(m, n, alpha, a, x, beta, y)
}

#[inline]
pub fn gemv_t<T: Blas>(m: usize, n: usize, alpha: T, a: &[T], x: &[T], beta: T, y: &mut [T]) {
    T::gemv_t(m, n, alpha, a, x, beta, y)
}

#[inline]
pub fn gemm<T: Blas>(
    m: usize,
    n: usize,
    k: usize,
    alpha: T,
    a: &[T],
    lda: usize,
    b: &[T],
    ldb: usize,
    beta: T,
    c: &mut [T],
    ldc: usize,
) {
    T::gemm(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

#[inline]
pub fn gemm_tn<T: Blas>(
    m: usize,
    n: usize,
    k: usize,
    alpha: T,
    a: &[T],
    lda: usize,
    b: &[T],
    ldb: usize,
    beta: T,
    c: &mut [T],
    ldc: usize,
) {
    T::gemm_tn(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

#[inline]
pub fn gemm_nt<T: Blas>(
    m: usize,
    n: usize,
    k: usize,
    alpha: T,
    a: &[T],
    lda: usize,
    b: &[T],
    ldb: usize,
    beta: T,
    c: &mut [T],
    ldc: usize,
) {
    T::gemm_nt(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

#[inline]
pub fn gemm_tt<T: Blas>(
    m: usize,
    n: usize,
    k: usize,
    alpha: T,
    a: &[T],
    lda: usize,
    b: &[T],
    ldb: usize,
    beta: T,
    c: &mut [T],
    ldc: usize,
) {
    T::gemm_tt(m, n, k, alpha, a, lda, b, ldb, beta, c, ldc)
}

#[inline]
pub fn matvec<T: Blas>(m: usize, n: usize, a: &[T], x: &[T]) -> Vec<T>
where
    T: Default + Blas,
{
    let mut y = vec![T::default(); m];
    T::gemv(m, n, T::one(), a, x, T::zero(), &mut y);
    y
}

#[inline]
pub fn matvec_t<T: Blas>(m: usize, n: usize, a: &[T], x: &[T]) -> Vec<T>
where
    T: Default + Blas,
{
    let mut y = vec![T::default(); n];
    T::gemv_t(m, n, T::one(), a, x, T::zero(), &mut y);
    y
}

#[inline]
pub fn matmul<T: Blas>(m: usize, n: usize, k: usize, a: &[T], b: &[T]) -> Vec<T>
where
    T: Default + Blas,
{
    let mut c = vec![T::default(); m * n];
    T::gemm(m, n, k, T::one(), a, k, b, n, T::zero(), &mut c, n);
    c
}

#[inline]
pub fn matmul_tn<T: Blas>(m: usize, n: usize, k: usize, a: &[T], b: &[T]) -> Vec<T>
where
    T: Default + Blas,
{
    let mut c = vec![T::default(); m * n];
    T::gemm_tn(m, n, k, T::one(), a, m, b, n, T::zero(), &mut c, n);
    c
}

#[inline]
pub fn matmul_nt<T: Blas>(m: usize, n: usize, k: usize, a: &[T], b: &[T]) -> Vec<T>
where
    T: Default + Blas,
{
    let mut c = vec![T::default(); m * n];
    T::gemm_nt(m, n, k, T::one(), a, k, b, k, T::zero(), &mut c, n);
    c
}

#[inline]
pub fn matmul_tt<T: Blas>(m: usize, n: usize, k: usize, a: &[T], b: &[T]) -> Vec<T>
where
    T: Default + Blas,
{
    let mut c = vec![T::default(); m * n];
    T::gemm_tt(m, n, k, T::one(), a, m, b, k, T::zero(), &mut c, n);
    c
}

pub use gemm::{dgemm, dgemm_nt, dgemm_tn, dgemm_tt, sgemm, sgemm_nt, sgemm_tn, sgemm_tt};
pub use gemm::{matmul_f32, matmul_f64};
