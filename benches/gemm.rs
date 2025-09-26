use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{thread_rng, Rng};

fn random_vec_f64(len: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

fn random_vec_f32(len: usize) -> Vec<f32> {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

fn bench_gemm_square_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemm_square_f64");

    // Test different matrix sizes from small to large
    for size in [16, 32, 64, 128, 256, 512, 768, 1024].iter() {
        let m = *size;
        let n = *size;
        let k = *size;

        let a = random_vec_f64(m * k);
        let b = random_vec_f64(k * n);
        let mut c_mat = vec![0.0; m * n];

        // Calculate FLOPS for throughput measurement
        let flops = 2 * m * n * k; // 2 ops per multiply-add
        group.throughput(Throughput::Elements(flops as u64));

        group.bench_with_input(BenchmarkId::new("dotzilla", size), size, |bench, _| {
            bench.iter(|| {
                dotzilla::gemm(m, n, k, 1.0, &a, k, &b, n, 0.0, black_box(&mut c_mat), n);
            });
        });
    }

    group.finish();
}

fn bench_gemm_rectangular_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemm_rectangular_f64");

    // Test various rectangular sizes common in ML workloads
    let sizes = [
        (512, 2048, 256), // Tall-skinny x short-wide
        (2048, 512, 256), // Short-wide x tall-skinny
        (1024, 1024, 64), // Large square with small k
        (64, 1024, 1024), // Small m with large n and k
        (784, 512, 128),  // Common FC layer size
        (128, 768, 512),  // Transformer-like dimensions
    ];

    for (m, n, k) in sizes.iter() {
        let a = random_vec_f64(m * k);
        let b = random_vec_f64(k * n);
        let mut c_mat = vec![0.0; m * n];

        let flops = 2 * m * n * k;
        group.throughput(Throughput::Elements(flops as u64));

        group.bench_with_input(
            BenchmarkId::new("dotzilla", format!("{}x{}x{}", m, n, k)),
            &(m, n, k),
            |bench, _| {
                bench.iter(|| {
                    dotzilla::gemm(
                        *m,
                        *n,
                        *k,
                        1.0,
                        &a,
                        *k,
                        &b,
                        *n,
                        0.0,
                        black_box(&mut c_mat),
                        *n,
                    );
                });
            },
        );
    }

    group.finish();
}

fn bench_gemm_small_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemm_small_f64");

    // Very small matrices that fit in L1 cache
    for size in [4, 8, 12, 16, 20, 24, 32].iter() {
        let m = *size;
        let n = *size;
        let k = *size;

        let a = random_vec_f64(m * k);
        let b = random_vec_f64(k * n);
        let mut c_mat = vec![0.0; m * n];

        let flops = 2 * m * n * k;
        group.throughput(Throughput::Elements(flops as u64));

        group.bench_with_input(BenchmarkId::new("dotzilla", size), size, |bench, _| {
            bench.iter(|| {
                dotzilla::gemm(m, n, k, 1.0, &a, k, &b, n, 0.0, black_box(&mut c_mat), n);
            });
        });
    }

    group.finish();
}

fn bench_gemm_transposed_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemm_transposed_f64");

    let size = 512;
    let m = size;
    let n = size;
    let k = size;

    let a = random_vec_f64(m * k);
    let b = random_vec_f64(k * n);
    let at = random_vec_f64(k * m); // For transposed A
    let bt = random_vec_f64(n * k); // For transposed B
    let mut c_mat = vec![0.0; m * n];

    let flops = 2 * m * n * k;
    group.throughput(Throughput::Elements(flops as u64));

    // Benchmark NN (no transpose)
    group.bench_function("NN", |bench| {
        bench.iter(|| {
            dotzilla::gemm(m, n, k, 1.0, &a, k, &b, n, 0.0, black_box(&mut c_mat), n);
        });
    });

    // Benchmark TN (A transposed)
    group.bench_function("TN", |bench| {
        bench.iter(|| {
            dotzilla::gemm_tn(
                m,
                n,
                k,
                1.0,
                &at,
                m, // Note: different stride for transposed
                &b,
                n,
                0.0,
                black_box(&mut c_mat),
                n,
            );
        });
    });

    // Benchmark NT (B transposed)
    group.bench_function("NT", |bench| {
        bench.iter(|| {
            dotzilla::gemm_nt(
                m,
                n,
                k,
                1.0,
                &a,
                k,
                &bt,
                k, // Note: different stride for transposed
                0.0,
                black_box(&mut c_mat),
                n,
            );
        });
    });

    // Benchmark TT (both transposed)
    group.bench_function("TT", |bench| {
        bench.iter(|| {
            dotzilla::gemm_tt(m, n, k, 1.0, &at, m, &bt, k, 0.0, black_box(&mut c_mat), n);
        });
    });

    group.finish();
}

fn bench_gemm_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemm_f32");

    for size in [64, 128, 256, 512, 1024].iter() {
        let m = *size;
        let n = *size;
        let k = *size;

        let a = random_vec_f32(m * k);
        let b = random_vec_f32(k * n);
        let mut c_mat = vec![0.0f32; m * n];

        let flops = 2 * m * n * k;
        group.throughput(Throughput::Elements(flops as u64));

        group.bench_with_input(BenchmarkId::new("dotzilla", size), size, |bench, _| {
            bench.iter(|| {
                dotzilla::gemm(
                    m,
                    n,
                    k,
                    1.0f32,
                    &a,
                    k,
                    &b,
                    n,
                    0.0f32,
                    black_box(&mut c_mat),
                    n,
                );
            });
        });
    }

    group.finish();
}

fn bench_matmul_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("matmul_simple");

    for size in [128, 256, 512].iter() {
        let m = *size;
        let n = *size;
        let k = *size;

        let a_f64 = random_vec_f64(m * k);
        let b_f64 = random_vec_f64(k * n);

        let a_f32 = random_vec_f32(m * k);
        let b_f32 = random_vec_f32(k * n);

        let flops = 2 * m * n * k;
        group.throughput(Throughput::Elements(flops as u64));

        group.bench_with_input(BenchmarkId::new("f64", size), size, |bench, _| {
            bench.iter(|| {
                let _c = dotzilla::matmul_f64(m, n, k, &a_f64, &b_f64);
            });
        });

        group.bench_with_input(BenchmarkId::new("f32", size), size, |bench, _| {
            bench.iter(|| {
                let _c = dotzilla::matmul_f32(m, n, k, &a_f32, &b_f32);
            });
        });
    }

    group.finish();
}

// Performance comparison with naive implementation
fn naive_gemm(m: usize, n: usize, k: usize, a: &[f64], b: &[f64], c: &mut [f64]) {
    for i in 0..m {
        for j in 0..n {
            let mut sum = 0.0;
            for kk in 0..k {
                sum += a[i * k + kk] * b[kk * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

fn bench_vs_naive(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemm_vs_naive");

    for size in [32, 64, 128, 256].iter() {
        let m = *size;
        let n = *size;
        let k = *size;

        let a = random_vec_f64(m * k);
        let b = random_vec_f64(k * n);
        let mut c_opt = vec![0.0; m * n];
        let mut c_naive = vec![0.0; m * n];

        let flops = 2 * m * n * k;
        group.throughput(Throughput::Elements(flops as u64));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |bench, _| {
            bench.iter(|| {
                dotzilla::gemm(m, n, k, 1.0, &a, k, &b, n, 0.0, black_box(&mut c_opt), n);
            });
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |bench, _| {
            bench.iter(|| {
                naive_gemm(m, n, k, &a, &b, black_box(&mut c_naive));
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_gemm_square_f64,
    bench_gemm_rectangular_f64,
    bench_gemm_small_f64,
    bench_gemm_transposed_f64,
    bench_gemm_f32,
    bench_matmul_simple,
    bench_vs_naive
);
criterion_main!(benches);
