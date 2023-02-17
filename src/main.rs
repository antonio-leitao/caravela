
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

use std::time::{Duration,Instant};
mod indexes;
mod utils;

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




fn main(){
    let mut index:indexes::Index32::Index32 = indexes::Index32::Index32::new(20);
    const N:usize = 200000;
    let query_points:usize =1000;
    const K:usize =16;
    let permutations = utils::random_permutations(N,K);
    
    for (data_point,permutation) in permutations.iter().enumerate(){
       index.insert(permutation_to_u128(&permutation),data_point);
    }

    let queries = utils::random_permutations(query_points,K);
    let mut total_time = Duration::new(0, 0);
    let mut count:u32 =0;

    for permutation in queries.iter(){
        let start_time = Instant::now();
        let candidates = index.query(permutation_to_u128(&permutation));
        
        let elapsed_time = start_time.elapsed();
        total_time += elapsed_time;
        count+=candidates.len() as u32;
     }
     let avg_time = total_time / query_points as u32;
     let avg_pool = count as f32 / query_points as f32;
     println!(
         "Average time per query: {:.2?}, Average pool size: {:.2}",
         avg_time, avg_pool
     );
}