mod hasher;
mod indexes;
mod utils;
use std::time::{Duration, Instant};

// fn run_metrics(n_samples:usize,n_queries:usize){
    
//     let mut index: indexes::Index = indexes::Index::new(vec![8,8,8,16,32,64]);
//     let permutations = utils::random_permutations(n_samples, 16);

//     for (data_point, permutation) in permutations.iter().enumerate() {
//         index.insert(utils::permutation_to_u128(&permutation), data_point);
//     }

//     let queries = utils::random_permutations(n_queries, 16);
//     let mut total_time = Duration::new(0, 0);
//     let mut count: u32 = 0;

//     for permutation in queries.iter() {
//         let start_time = Instant::now();
//         let candidates = index.query(utils::permutation_to_u128(&permutation));

//         let elapsed_time = start_time.elapsed();
//         total_time += elapsed_time;
//         count += candidates.len() as u32;
//     }
//     let avg_time = total_time.as_millis() as f32 / n_queries as f32;
//     let avg_pool = count as f32 / n_queries as f32;
//     println!("Average time per query: {} ms",avg_time);
//     println!("Candidates pool size: {}",avg_pool);
// }




// fn main(){
//     run_metrics(100_000,1_000);
// }

use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;

use crate::hasher::MaskedHasher;

fn main(){
    println!("----- HashMap (with its default SipHash hasher) -----------");
let hm: HashMap<i32, i32> = (0..1_000_000).map(|i| (i, i)).collect();
for _k in 0..6 {
    let t0 = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..1_000_000 {
        if let Some(x) = hm.get(&i) {
            sum += *x as i64;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
}

println!("----- HashMap (with HashBrown hasher) -----------");
let hm: hashbrown::HashMap<i32, i32> = (0..1_000_000).map(|i| (i, i)).collect();
for _k in 0..6 {
    let t0 = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..1_000_000 {
        if let Some(x) = hm.get(&i) {
            sum += *x as i64;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
}


println!("----- HashMap/BuildNoHashHasher (nohash-hasher 0.2.0) -----");
let hm: HashMap<u64, u64, BuildNoHashHasher<u64>> = (0..1_000_000).map(|i| (i, i)).collect();


for _k in 0..6 {
    let t0 = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..1_000_000 {
        
        if let Some(x) = hm.get(&i) {
            sum += *x as i64;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
}

println!("----- HashMap/CustomHasher (nohash-hasher 0.2.0) -----");
let mut hm: HashMap<u128, u32, MaskedHasher> = HashMap::with_hasher(MaskedHasher::new(32));
    for i in 0..1_000_0{
        hm.insert(i,i as u32);
    }

for _k in 0..6 {
    let t0 = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..1_000_0 {
        
        if let Some(x) = hm.get(&i) {
            sum += *x as i64;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
}

}

