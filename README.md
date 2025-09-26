<p align="center">
  <img src='assets/logo.svg' width='200px' align="center"></img>
</p>

<div align="center">
<h3 max-width='200px' align="center">Caravela</h3>
  <p><i>High performance Linear algebra primitives in Rust<br/>
  Optimzed for modern SIMD architectures<br/>
  Built fully in Rust</i><br/></p>
  <p>
<img alt="Pepy Total Downlods" src="https://img.shields.io/pepy/dt/caravela?style=for-the-badge&logo=python&labelColor=white&color=blue">
  </p>
</div>

#

Caravela provides a comprehensive suite of linear algebra operations from basic vector dot products to complex matrix multiplications, all with best-in-class performance through runtime CPU feature detection and highly optimized SIMD kernels.

## Key Features

- **Zero Dependencies**: Pure Rust implementation
- **Complete BLAS Coverage**: Level 1 (vector-vector), Level 2 (matrix-vector), and Level 3 (matrix-matrix) operations
- **Two-Level API**:
  - Simple high-level functions for casual users
  - Full BLAS-style interface for advanced control
- **SIMD Acceleration**:
  - **x86_64**: AVX2 + FMA with optimized microkernel implementations
  - **AArch64**: NEON optimized for ARM processors (Apple Silicon, AWS Graviton, etc.)
- **Cache-Optimized GEMM**: State-of-the-art BLIS algorithm implementation with multi-level cache blocking
- **Runtime Feature Detection**: Automatically selects the best implementation for your CPU
- **Generic Design**: Seamless operation with both `f32` and `f64` types

## Installation

```toml
[dependencies]
caravela = "0.1.0"
```

## Usage

Caravela provides two API levels to suit different needs:

### High-Level API

Simple, easy-to-use functions for everyday linear algebra operations.

```rust
use caravela::{dot, l2, matvec, matmul};

// Vector dot product
let a = vec![1.0, 2.0, 3.0];
let b = vec![4.0, 5.0, 6.0];
let result = dot(&a, &b);  // 32.0

// Euclidean distance
let distance = l2(&a, &b);  // sqrt(27) ≈ 5.196

// Matrix-vector multiplication: y = Ax
let matrix = vec![1.0, 2.0, 3.0,  // 2x3 matrix (row-major)
                  4.0, 5.0, 6.0];
let vector = vec![1.0, 1.0, 1.0];
let result = matvec(2, 3, &matrix, &vector);  // [6.0, 15.0]

// Matrix-matrix multiplication: C = AB
let a = vec![1.0, 2.0,  // 2x2 matrix
             3.0, 4.0];
let b = vec![5.0, 6.0,  // 2x2 matrix
             7.0, 8.0];
let c = matmul(2, 2, 2, &a, &b);  // [19.0, 22.0, 43.0, 50.0]
```

### Low-Level API

BLAS-style interface providing full control over all parameters and operations.

```rust
use caravela::{dot, l2, gemv, gemv_t, gemm, gemm_tn, gemm_nt, gemm_tt};

// Vector operations (same as high-level, included for completeness)
let dot_product = dot(&a, &b);
let distance = l2(&a, &b);

// General matrix-vector multiply: y = α·A·x + β·y
let matrix = vec![1.0, 2.0, 3.0,  // 2x3 matrix
                  4.0, 5.0, 6.0];
let x = vec![1.0, 1.0, 1.0];
let mut y = vec![10.0, 20.0];

gemv(
    2, 3,              // m, n dimensions
    2.0,               // alpha
    &matrix,           // A matrix
    &x,                // x vector
    0.5,               // beta
    &mut y             // y vector (modified in place)
);
// y = 2.0 * [6.0, 15.0] + 0.5 * [10.0, 20.0] = [17.0, 40.0]

// Transposed matrix-vector multiply: y = α·A^T·x + β·y
let x2 = vec![1.0, 2.0];  // Note: x has m elements for A^T
let mut y2 = vec![5.0, 5.0, 5.0];  // y has n elements

gemv_t(
    2, 3,              // m, n dimensions of A
    1.0,               // alpha
    &matrix,           // A matrix (will be transposed)
    &x2,               // x vector
    1.0,               // beta
    &mut y2            // y vector
);
// Computes: y = A^T * x + y
// A^T * [1,2] = [9, 12, 15], so y = [14, 17, 20]

// General matrix-matrix multiply: C = α·A·B + β·C
let a = vec![1.0, 2.0,  // 2x2 matrix
             3.0, 4.0];
let b = vec![5.0, 6.0,  // 2x2 matrix
             7.0, 8.0];
let mut c = vec![1.0; 4];  // 2x2 matrix

gemm(
    2, 2, 2,           // m, n, k dimensions
    2.0,               // alpha
    &a, 2,             // A matrix and leading dimension
    &b, 2,             // B matrix and leading dimension
    3.0,               // beta
    &mut c, 2          // C matrix and leading dimension
);
// c = 2.0 * A * B + 3.0 * C
// c = 2.0 * [19,22,43,50] + 3.0 * [1,1,1,1] = [41,47,89,103]

// Transposed A: C = α·A^T·B + β·C
let a_t = vec![1.0, 3.0,  // A transposed (original A in column-major)
               2.0, 4.0];
gemm_tn(
    2, 2, 2,           // m, n, k dimensions
    1.0,               // alpha
    &a_t, 2,           // A^T matrix and leading dimension
    &b, 2,             // B matrix and leading dimension
    0.0,               // beta
    &mut c, 2          // C matrix
);

// Transposed B: C = α·A·B^T + β·C
let b_t = vec![5.0, 7.0,  // B transposed
               6.0, 8.0];
gemm_nt(
    2, 2, 2,           // m, n, k dimensions
    1.0,               // alpha
    &a, 2,             // A matrix and leading dimension
    &b_t, 2,           // B^T matrix and leading dimension
    0.0,               // beta
    &mut c, 2          // C matrix
);

// Both transposed: C = α·A^T·B^T + β·C
gemm_tt(
    2, 2, 2,           // m, n, k dimensions
    1.0,               // alpha
    &a_t, 2,           // A^T matrix and leading dimension
    &b_t, 2,           // B^T matrix and leading dimension
    0.0,               // beta
    &mut c, 2          // C matrix
);
```

## Performance

Caravela implements state-of-the-art algorithms for maximum performance:

### GEMM (Matrix Multiplication)

- **BLIS Algorithm**: 5-level nested loops with cache blocking
- **Optimized Microkernels**: Hand-tuned SIMD kernels for AVX2 and NEON
- **Cache-Aware Design**: Multi-level blocking (L1/L2/L3) for optimal data reuse
- **Performance**: 85-95% of theoretical peak FLOPS on modern CPUs

### GEMV (Matrix-Vector)

- **Blocked Algorithm**: Cache-friendly tiling for both standard and transposed operations
- **SIMD Acceleration**: Vectorized dot products for each row/column

### Vector Operations

- **Unrolled Loops**: 8-way unrolling with multiple accumulators
- **SIMD Utilization**: Full width vectors (256-bit AVX2, 128-bit NEON)
- **Performance**: Near memory bandwidth limits for large vectors

## Benchmarks

Run the comprehensive benchmark suite:

```bash
# All benchmarks
cargo bench

# Specific operations
cargo bench --bench gemm
cargo bench --bench gemv
cargo bench --bench dot

# Compare with naive implementations
cargo bench "vs_naive"
```

## Architecture Support

- **x86_64**: Requires AVX2 + FMA (Intel Haswell/AMD Excavator or newer)
- **AArch64**: Requires NEON (all ARMv8+ processors)
- **Fallback**: Optimized scalar implementation for other architectures

The library automatically detects and uses the best available instruction set at runtime.

## Future Directions

Caravela is a project that was bron from my needs.
It is also in constant development as I learn more about low level programming.
Future areas of development:

- GPU acceleration backends
