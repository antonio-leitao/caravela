
use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;
use std::time::Instant;
mod try_hasher;
use rand::{Rng, seq::IteratorRandom};
use std::collections::HashSet;

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

#[test]
fn test_random_indexes(){
    let n_bits:usize = 64;
    let index_list = generate_non_repeated_sequence(n_bits);
    assert_eq!(index_list.len(),n_bits);
    let x = extract_bits_at_indexes(&index_list, u128::MAX);
    assert_eq!(x.count_ones(),n_bits as u32);
    let x = extract_bits_at_indexes(&index_list, u128::MIN);
    assert_eq!(x.count_zeros(),n_bits as u32);


}

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

    // println!("----- HashMap/IndexHasher (u64) -----");
    // let mut hm:try_hasher::IndexHasher<u64> = try_hasher::IndexHasher::new(generate_non_repeated_sequence(64)); 
    
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

    println!("----- Mask Overhead -----");
    let index_list = generate_non_repeated_sequence(64); 
    println!("{:?}", index_list);

    // for _k in 0..6 {
    //     let t0 = Instant::now();
    //     let mut sum: i64 = 0;
    //     for i in 0..1_000_000u64 {

    //         let x = extract_bits_at_indexes(&index_list, u128::from(i));
    //         sum += x as i64;

    //     }
    //     let elapsed = t0.elapsed().as_secs_f64();
    //     println!("The sum is: {}. Time elapsed: {:.3} sec", sum, elapsed);
    // }
    
}
