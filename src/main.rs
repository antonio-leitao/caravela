
// use std::time::Instant;
// use rand::Rng;

// fn select_32_bits() -> impl Fn(u128) -> u32 {
//     let mut rng = rand::thread_rng();
//     let shift = rng.gen_range(0..97);
//     let mask = 0xFFFF_FFFFu128 << shift;
//     move |v| ((v & mask) >> shift) as u32
// }

// fn main() {
//     let value: u128 = 0x1234_5678_9ABC_DEF0_1111_2222_3333_4444;
//     let random_hash = select_32_bits();
//     let start = Instant::now();
//     for _ in 0..1_000_000 {
//         _=random_hash(value)
//     }
//     println!("{:?}",start.elapsed());

// }

use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;
use std::time::Instant;

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

}

