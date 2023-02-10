use std::time::Instant;


fn flip_bit(num: u8, index: u8) -> u8 {
    let mask = 1u8 << index;
    num ^ mask
}

fn main() {
    let num = 0b10101100;
    let index = 3;
    let start = Instant::now();
    for _ in 0..1_000_000{
        let result = flip_bit(num, index);
    }
    let elapsed = start.elapsed();
    //println!("Before flipping bit: {:08b}", num);
    //println!("After flipping bit:  {:08b}", result);
    println!("Time elapsed: {:?}", elapsed);
}