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
fn naive_dot_f64(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}

fn naive_dot_f32(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}

fn naive_l2_f64(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum.sqrt()
}

fn naive_l2_f32(a: &[f32], b: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum.sqrt()
}

fn bench_dot_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_f64");

    // Test various sizes from small to large
    let sizes = [16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];

    for size in sizes.iter() {
        let a = random_vec_f64(*size);
        let b = random_vec_f64(*size);

        // Calculate FLOPS (2 ops per element: multiply and add)
        group.throughput(Throughput::Elements(*size as u64 * 2));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |bench, _| {
            bench.iter(|| caravela::dot(&a, &b));
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |bench, _| {
            bench.iter(|| naive_dot_f64(&a, &b));
        });
    }

    group.finish();
}

fn bench_dot_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_f32");

    let sizes = [16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];

    for size in sizes.iter() {
        let a = random_vec_f32(*size);
        let b = random_vec_f32(*size);

        group.throughput(Throughput::Elements(*size as u64 * 2));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |bench, _| {
            bench.iter(|| caravela::dot(&a, &b));
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |bench, _| {
            bench.iter(|| naive_dot_f32(&a, &b));
        });
    }

    group.finish();
}

fn bench_l2_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("l2_distance_f64");

    let sizes = [16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];

    for size in sizes.iter() {
        let a = random_vec_f64(*size);
        let b = random_vec_f64(*size);

        // 3 ops per element: subtract, multiply, add, plus 1 sqrt at the end
        group.throughput(Throughput::Elements(*size as u64 * 3 + 1));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |bench, _| {
            bench.iter(|| caravela::l2sq(&a, &b));
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |bench, _| {
            bench.iter(|| naive_l2_f64(&a, &b));
        });
    }

    group.finish();
}

fn bench_l2_f32(c: &mut Criterion) {
    let mut group = c.benchmark_group("l2_distance_f32");

    let sizes = [16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];

    for size in sizes.iter() {
        let a = random_vec_f32(*size);
        let b = random_vec_f32(*size);

        group.throughput(Throughput::Elements(*size as u64 * 3 + 1));

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |bench, _| {
            bench.iter(|| caravela::l2sq(&a, &b));
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |bench, _| {
            bench.iter(|| naive_l2_f32(&a, &b));
        });
    }

    group.finish();
}

fn bench_dot_alignment(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_alignment_impact");

    let size = 1024;

    // Aligned vectors
    let a_aligned = random_vec_f64(size);
    let b_aligned = random_vec_f64(size);

    // Misaligned vectors (skip first element)
    let a_misaligned = random_vec_f64(size + 1);
    let b_misaligned = random_vec_f64(size + 1);

    group.bench_function("aligned", |bench| {
        bench.iter(|| caravela::dot(&a_aligned, &b_aligned));
    });

    group.bench_function("misaligned", |bench| {
        bench.iter(|| caravela::dot(&a_misaligned[1..], &b_misaligned[1..]));
    });

    group.finish();
}

fn bench_dot_small_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_small_sizes");

    // Very small sizes where SIMD overhead might matter
    let sizes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    for size in sizes.iter() {
        let a = random_vec_f64(*size);
        let b = random_vec_f64(*size);

        group.bench_with_input(BenchmarkId::new("optimized", size), size, |bench, _| {
            bench.iter(|| caravela::dot(&a, &b));
        });

        group.bench_with_input(BenchmarkId::new("naive", size), size, |bench, _| {
            bench.iter(|| naive_dot_f64(&a, &b));
        });
    }

    group.finish();
}

fn bench_dot_cache_effects(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_cache_effects");

    // Test sizes that fit in different cache levels
    // L1: ~32KB = 4K doubles
    // L2: ~256KB = 32K doubles
    // L3: ~8MB = 1M doubles
    let sizes = [
        ("L1", 4_000),
        ("L2", 32_000),
        ("L3", 1_000_000),
        ("RAM", 10_000_000),
    ];

    for (name, size) in sizes.iter() {
        let a = random_vec_f64(*size);
        let b = random_vec_f64(*size);

        group.throughput(Throughput::Bytes((*size as u64) * 8 * 2)); // 8 bytes per f64, 2 vectors

        group.bench_with_input(BenchmarkId::new("caravela", name), size, |bench, _| {
            bench.iter(|| caravela::dot(&a, &b));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_dot_f64,
    bench_dot_f32,
    bench_l2_f64,
    bench_l2_f32,
    bench_dot_alignment,
    bench_dot_small_sizes,
    bench_dot_cache_effects
);
criterion_main!(benches);
