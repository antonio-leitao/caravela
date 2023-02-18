// use std::time::Instant;

// fn main() {
//     let value: u128 = 0x1234_5678_9ABC_DEF0_1111_2222_3333_4444;
//     let random_hash = select_32_bits();
//     let start = Instant::now();
//     for _ in 0..1_000_000 {
//         _=random_hash(value)
//     }
//     println!("{:?}",start.elapsed());

// }

use std::time::{Duration, Instant};
mod indexes;
mod utils;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn permutation_to_u128(vector: &[usize]) -> u128 {
    let mut packed: u128 = 0;
    for i in 0..vector.len() {
        for j in i + 1..vector.len() {
            packed <<= 1;
            if vector[i] > vector[j] {
                packed |= 1;
            }
        }
    }
    packed
}

fn run_metrics64(n_samples: usize, n_queries: usize, n_hashes: usize) -> (f32, f32) {
    let mut index: indexes::Index64::Index64 = indexes::Index64::Index64::new(n_hashes);
    let permutations = utils::random_permutations(n_samples, 16);

    for (data_point, permutation) in permutations.iter().enumerate() {
        index.insert(permutation_to_u128(&permutation), data_point);
    }

    let queries = utils::random_permutations(n_queries, 16);
    let mut total_time = Duration::new(0, 0);
    let mut count: u32 = 0;

    for permutation in queries.iter() {
        let start_time = Instant::now();
        let candidates = index.query(permutation_to_u128(&permutation));

        let elapsed_time = start_time.elapsed();
        total_time += elapsed_time;
        count += candidates.len() as u32;
    }
    let avg_time = total_time.as_millis() as f32 / n_queries as f32;
    let avg_pool = count as f32 / n_queries as f32;
    (avg_time, avg_pool)
}

fn run_metrics32(n_samples: usize, n_queries: usize, n_hashes: usize) -> (f32, f32) {
    let mut index: indexes::Index32::Index32 = indexes::Index32::Index32::new(n_hashes);
    let permutations = utils::random_permutations(n_samples, 16);

    for (data_point, permutation) in permutations.iter().enumerate() {
        index.insert(permutation_to_u128(&permutation), data_point);
    }

    let queries = utils::random_permutations(n_queries, 16);
    let mut total_time = Duration::new(0, 0);
    let mut count: u32 = 0;

    for permutation in queries.iter() {
        let start_time = Instant::now();
        let candidates = index.query(permutation_to_u128(&permutation));

        let elapsed_time = start_time.elapsed();
        total_time += elapsed_time;
        count += candidates.len() as u32;
    }
    let avg_time = total_time.as_millis() as f32 / n_queries as f32;
    let avg_pool = count as f32 / n_queries as f32;
    (avg_time, avg_pool)
}

fn run_metrics16(n_samples: usize, n_queries: usize, n_hashes: usize) -> (f32, f32) {
    let mut index: indexes::Index16::Index16 = indexes::Index16::Index16::new(n_hashes);
    let permutations = utils::random_permutations(n_samples, 16);

    for (data_point, permutation) in permutations.iter().enumerate() {
        index.insert(permutation_to_u128(&permutation), data_point);
    }

    let queries = utils::random_permutations(n_queries, 16);
    let mut total_time = Duration::new(0, 0);
    let mut count: u32 = 0;

    for permutation in queries.iter() {
        let start_time = Instant::now();
        let candidates = index.query(permutation_to_u128(&permutation));

        let elapsed_time = start_time.elapsed();
        total_time += elapsed_time;
        count += candidates.len() as u32;
    }
    let avg_time = total_time.as_millis() as f32 / n_queries as f32;
    let avg_pool = count as f32 / n_queries as f32;
    (avg_time, avg_pool)
}

fn run_metrics8(n_samples: usize, n_queries: usize, n_hashes: usize) -> (f32, f32) {
    let mut index: indexes::Index8::Index8 = indexes::Index8::Index8::new(n_hashes);
    let permutations = utils::random_permutations(n_samples, 16);

    for (data_point, permutation) in permutations.iter().enumerate() {
        index.insert(permutation_to_u128(&permutation), data_point);
    }

    let queries = utils::random_permutations(n_queries, 16);
    let mut total_time = Duration::new(0, 0);
    let mut count: u32 = 0;

    for permutation in queries.iter() {
        let start_time = Instant::now();
        let candidates = index.query(permutation_to_u128(&permutation));

        let elapsed_time = start_time.elapsed();
        total_time += elapsed_time;
        count += candidates.len() as u32;
    }
    let avg_time = total_time.as_millis() as f32 / n_queries as f32;
    let avg_pool = count as f32 / n_queries as f32;
    (avg_time, avg_pool)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_path("output.csv")?;

    writer.write_record(&["indextype", "n_samples", "time", "pool", "n_hashes"])?;

    let n_queries = 1_000;
    for n_samples in [
        1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 100_000, 200_000,
    ] {
        for n_hashes in [1, 2, 5, 10, 20, 50, 100] {
            let (time, pool) = run_metrics64(n_samples, n_queries, n_hashes);
            writer.write_record(&[
                "64",
                &n_samples.to_string(),
                &time.to_string(),
                &pool.to_string(),
                &n_hashes.to_string(),
            ])?;

            let (time, pool) = run_metrics32(n_samples, n_queries, n_hashes);
            writer.write_record(&[
                "32",
                &n_samples.to_string(),
                &time.to_string(),
                &pool.to_string(),
                &n_hashes.to_string(),
            ])?;

            let (time, pool) = run_metrics16(n_samples, n_queries, n_hashes);
            writer.write_record(&[
                "16",
                &n_samples.to_string(),
                &time.to_string(),
                &pool.to_string(),
                &n_hashes.to_string(),
            ])?;

            let (time, pool) = run_metrics8(n_samples, n_queries, n_hashes);
            writer.write_record(&[
                "8",
                &n_samples.to_string(),
                &time.to_string(),
                &pool.to_string(),
                &n_hashes.to_string(),
            ])?;
        }
        println!{"finished n_samples {:?}", n_samples};
    }

    writer.flush()?;

    Ok(())
}
