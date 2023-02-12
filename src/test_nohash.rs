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

