
use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;
use std::time::Instant;
mod try_hasher;
use rand::{Rng, seq::IteratorRandom};
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_non_repeated_sequence(n:usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut used_numbers = HashSet::new();
    let mut numbers:Vec<u8> = Vec::new();
    while numbers.len() < n {
        let number = (0..128).choose(&mut rng).unwrap();
        if used_numbers.insert(number) {
            numbers.push(number);
        }
    }
    numbers
}

fn extract_bits_at_indexes(indexes: &[u8], input: u128) -> u64 {
    let mut result: u64 = 0;
    for (i, &index) in indexes.iter().enumerate() {
        if input & (1u128 << index) != 0 {
            result |= 1 << i;
        }
    }
    result
}

// fn random_u128_mask(num_ones: usize) -> u128 {
//     let mut rng = thread_rng();
//     let mut indices: Vec<usize> = (0..128 as usize).collect();
//     indices.shuffle(&mut rng);
//     indices.truncate(num_ones as usize);

//     let mut bits: u128 = 0;
//     for i in indices {
//         bits |= 1 << i;
//     }
//     bits
// }

// fn apply_mask_and_hash(mask: u128, input: u128) -> u64 {
//     let masked_input = mask & input;
//     hash_u128_to_u64(masked_input)
// }

// fn hash_u128_to_u64(x: u128) -> u64 {
//     let h1: u64 = x as u64;
//     let h2: u64 = (x >> 64) as u64;
//     h1 ^ h2
// }

// #[test]
// fn test_random_indexes(){
//     let n_bits:usize = 54;
//     let random_mask = random_u128_mask(n_bits);
//     assert_eq!(random_mask.count_ones(),n_bits as u32,"generate appropriate vector");
// }

fn random_u128_mask(num_ones: usize) -> u128 {
    let mut rng = thread_rng();
    let mut indices: Vec<usize> = (0..128 as usize).collect();
    indices.shuffle(&mut rng);
    indices.truncate(num_ones as usize);

    let mut bits: u128 = 0;
    for i in indices {
        bits |= 1 << i;
    }
    bits
}

fn masked_hash(mask: u128, input: u128) -> u64 {
    let masked_input = mask & input;
    let h1: u64 = masked_input as u64;
    let h2: u64 = (masked_input >> 64) as u64;
    h1 ^ h2
}

fn main(){
    println!("----- Random XOR mask -----------");
    let n_bits:usize = 64;
    for _k in 0..6 {
        let mask = random_u128_mask(n_bits);
        let t0 = Instant::now();
        let mut sum: u128 = 0;
        for input in 0..1_000_000 {
            let x = masked_hash(mask, input);
                sum += x as u128;
            
        }
        let elapsed = t0.elapsed().as_secs_f64();
        println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    }

    println!("----- Direct index extraction -----------");
    let n_bits:usize = 64;
    for _k in 0..6 {
        let index_list = generate_non_repeated_sequence(n_bits);
        let t0 = Instant::now();
        let mut sum: u128 = 0;
        for input in 0..1_000_000 {
            let x = extract_bits_at_indexes(&index_list, input);
                sum += x as u128;
            
        }
        let elapsed = t0.elapsed().as_secs_f64();
        println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    }



    
    // println!("----- HashMap (with its default SipHash hasher) -----------");
    // let hm: HashMap<i32, i32> = (0..1_000_000).map(|i| (i, i)).collect();
    // for _k in 0..6 {
    //     let t0 = Instant::now();
    //     let mut sum: i64 = 0;
    //     for i in 0..1_000_000 {
    //         if let Some(x) = hm.get(&i) {
    //             sum += *x as i64;
    //         }
    //     }
    //     let elapsed = t0.elapsed().as_secs_f64();
    //     println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    // }
    
    // println!("----- HashMap (with HashBrown hasher) -----------");
    // let hm: hashbrown::HashMap<i32, i32> = (0..1_000_000).map(|i| (i, i)).collect();
    // for _k in 0..6 {
    //     let t0 = Instant::now();
    //     let mut sum: i64 = 0;
    //     for i in 0..1_000_000 {
    //         if let Some(x) = hm.get(&i) {
    //             sum += *x as i64;
    //         }
    //     }
    //     let elapsed = t0.elapsed().as_secs_f64();
    //     println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    // }
    
    
    // println!("----- HashMap/BuildNoHashHasher (nohash-hasher 0.2.0) -----");
    // let hm: HashMap<u64, u64, BuildNoHashHasher<u64>> = (0..1_000_000).map(|i| (i, i)).collect();
    
    
    // for _k in 0..6 {
    //     let t0 = Instant::now();
    //     let mut sum: i64 = 0;
    //     for i in 0..1_000_000 {
            
    //         if let Some(x) = hm.get(&i) {
    //             sum += *x as i64;
    //         }
    //     }
    //     let elapsed = t0.elapsed().as_secs_f64();
    //     println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    // }

    // println!("----- HashMap/IndexHasher (u64) -----");
    // let mut hm:try_hasher::IndexHasher<u64> = try_hasher::IndexHasher::new(54); 
    
    // for i in 0..1_000_000{
    //     hm.insert(u128::from(i),i)
    // }

    // for _k in 0..6 {
    //     let t0 = Instant::now();
    //     let mut sum: i64 = 0;
    //     for i in 0..1_000_000u64 {
            
    //         if let Some(x) = hm.get(u128::from(i)) {
    //             sum += *x as i64;
    //         }
    //     }
    //     let elapsed = t0.elapsed().as_secs_f64();
    //     println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    // }

    
}
