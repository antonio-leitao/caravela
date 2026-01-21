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

// Naive implementations for comparison
fn naive_gemv_f64(m: usize, n: usize, alpha: f64, a: &[f64], x: &[f64], beta: f64, y: &mut [f64]) {
    for i in 0..m {
        let mut sum = 0.0;
        for j in 0..n {
            sum += a[i * n + j] * x[j];
        }
        y[i] = alpha * sum + beta * y[i];
    }
}

fn naive_gemv_f32(m: usize, n: usize, alpha: f32, a: &[f32], x: &[f32], beta: f32, y: &mut [f32]) {
    for i in 0..m {
        let mut sum = 0.0;
        for j in 0..n {
            sum += a[i * n + j] * x[j];
        }
        y[i] = alpha * sum + beta * y[i];
    }
}

fn naive_gemv_t_f64(
    m: usize,
    n: usize,
    alpha: f64,
    a: &[f64],
    x: &[f64],
    beta: f64,
    y: &mut [f64],
) {
    // Scale y by beta first
    for j in 0..n {
        y[j] *= beta;
    }

    // Compute A^T * x
    for i in 0..m {
        for j in 0..n {
            y[j] += alpha * a[i * n + j] * x[i];
        }
    }
}

fn naive_gemv_t_f32(
    m: usize,
    n: usize,
    alpha: f32,
    a: &[f32],
    x: &[f32],
    beta: f32,
    y: &mut [f32],
) {
    for j in 0..n {
        y[j] *= beta;
    }

    for i in 0..m {
        for j in 0..n {
            y[j] += alpha * a[i * n + j] * x[i];
        }
    }
}

fn bench_gemv_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemv_f64");

    for size in [64, 128, 256, 512, 1024].iter() {
        let m = *size;
        let n = *size;

        let a: Vec<f64> = random_vec_f64(m * n);
        let x: Vec<f64> = random_vec_f64(n);
        let mut y: Vec<f64> = vec![0.0; m];
        let mut y_naive: Vec<f64> = vec![0.0; m];

        // FLOPS: 2*m*n for the matrix-vector multiply
        group.throughput(Throughput::Elements((2 * m * n) as u64));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |b, _| {
            b.iter(|| {
                caravela::gemv(m, n, 1.0, &a, &x, 0.0, black_box(&mut y));
            });
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |b, _| {
            b.iter(|| {
                naive_gemv_f64(m, n, 1.0, &a, &x, 0.0, black_box(&mut y_naive));
            });
        });
    }

    group.finish();
}

fn bench_gemv_t_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemv_t_f64");

    for size in [64, 128, 256, 512, 1024].iter() {
        let m = *size;
        let n = *size;

        let a: Vec<f64> = random_vec_f64(m * n);
        let x: Vec<f64> = random_vec_f64(m);
        let mut y: Vec<f64> = vec![0.0; n];
        let mut y_naive: Vec<f64> = vec![0.0; n];

        group.throughput(Throughput::Elements((2 * m * n) as u64));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |b, _| {
            b.iter(|| {
                caravela::gemv_t(m, n, 1.0, &a, &x, 0.0, black_box(&mut y));
            });
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |b, _| {
            b.iter(|| {
                naive_gemv_t_f64(m, n, 1.0, &a, &x, 0.0, black_box(&mut y_naive));
            });
        });
    }

    group.finish();
}

fn bench_gemv_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemv_f32");

    for size in [64, 128, 256, 512, 1024].iter() {
        let m = *size;
        let n = *size;

        let a: Vec<f32> = random_vec_f32(m * n);
        let x: Vec<f32> = random_vec_f32(n);
        let mut y: Vec<f32> = vec![0.0; m];
        let mut y_naive: Vec<f32> = vec![0.0; m];

        group.throughput(Throughput::Elements((2 * m * n) as u64));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |b, _| {
            b.iter(|| {
                caravela::gemv(m, n, 1.0f32, &a, &x, 0.0f32, black_box(&mut y));
            });
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |b, _| {
            b.iter(|| {
                naive_gemv_f32(m, n, 1.0f32, &a, &x, 0.0f32, black_box(&mut y_naive));
            });
        });
    }

    group.finish();
}

fn bench_gemv_t_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemv_t_f32");

    for size in [64, 128, 256, 512, 1024].iter() {
        let m = *size;
        let n = *size;

        let a: Vec<f32> = random_vec_f32(m * n);
        let x: Vec<f32> = random_vec_f32(m);
        let mut y: Vec<f32> = vec![0.0; n];
        let mut y_naive: Vec<f32> = vec![0.0; n];

        group.throughput(Throughput::Elements((2 * m * n) as u64));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |b, _| {
            b.iter(|| {
                caravela::gemv_t(m, n, 1.0f32, &a, &x, 0.0f32, black_box(&mut y));
            });
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |b, _| {
            b.iter(|| {
                naive_gemv_t_f32(m, n, 1.0f32, &a, &x, 0.0f32, black_box(&mut y_naive));
            });
        });
    }

    group.finish();
}

fn bench_gemv_rectangular(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemv_rectangular");

    // Test various rectangular sizes
    let sizes = [
        (512, 128), // Tall-skinny
        (128, 512), // Short-wide
        (1024, 64), // Very tall-skinny
        (64, 1024), // Very short-wide
    ];

    for (m, n) in sizes.iter() {
        let a: Vec<f64> = random_vec_f64(m * n);
        let x: Vec<f64> = random_vec_f64(*n);
        let mut y: Vec<f64> = vec![0.0; *m];

        group.throughput(Throughput::Elements((2 * m * n) as u64));

        group.bench_with_input(
            BenchmarkId::new("optimized", format!("{}x{}", m, n)),
            &(m, n),
            |b, _| {
                b.iter(|| {
                    caravela::gemv(*m, *n, 1.0, &a, &x, 0.0, black_box(&mut y));
                });
            },
        );

        let mut y_naive = vec![0.0; *m];
        group.bench_with_input(
            BenchmarkId::new("naive", format!("{}x{}", m, n)),
            &(m, n),
            |b, _| {
                b.iter(|| {
                    naive_gemv_f64(*m, *n, 1.0, &a, &x, 0.0, black_box(&mut y_naive));
                });
            },
        );
    }

    group.finish();
}

fn bench_gemv_alpha_beta(c: &mut Criterion) {
    let mut group = c.benchmark_group("gemv_alpha_beta");

    let size = 512;
    let m = size;
    let n = size;

    let a: Vec<f64> = random_vec_f64(m * n);
    let x: Vec<f64> = random_vec_f64(n);
    let mut y: Vec<f64> = random_vec_f64(m);

    // Test with different alpha/beta values
    let params = [
        ("standard", 1.0, 0.0),   // Standard multiply
        ("accumulate", 1.0, 1.0), // y = Ax + y
        ("scaled", 2.5, 3.0),     // y = 2.5*Ax + 3.0*y
        ("replace", 1.0, 0.5),    // y = Ax + 0.5*y
    ];

    for (name, alpha, beta) in params.iter() {
        group.bench_function(*name, |b| {
            b.iter(|| {
                caravela::gemv(m, n, *alpha, &a, &x, *beta, black_box(&mut y));
            });
        });
    }

    group.finish();
}

fn bench_matvec_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("matvec_simple");

    for size in [128, 256, 512].iter() {
        let m = *size;
        let n = *size;

        let a_f64 = random_vec_f64(m * n);
        let x_f64 = random_vec_f64(n);

        let a_f32 = random_vec_f32(m * n);
        let x_f32 = random_vec_f32(n);

        group.throughput(Throughput::Elements((2 * m * n) as u64));

        group.bench_with_input(BenchmarkId::new("f64", size), size, |b, _| {
            b.iter(|| {
                let _y = caravela::matvec::<f64>(m, n, &a_f64, &x_f64);
            });
        });

        group.bench_with_input(BenchmarkId::new("f32", size), size, |b, _| {
            b.iter(|| {
                let _y = caravela::matvec::<f32>(m, n, &a_f32, &x_f32);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_gemv_f64,
    bench_gemv_t_f64,
    bench_gemv_f32,
    bench_gemv_t_f32,
    bench_gemv_rectangular,
    bench_gemv_alpha_beta,
    bench_matvec_simple
);
criterion_main!(benches);
