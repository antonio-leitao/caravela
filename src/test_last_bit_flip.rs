
use std::time::Instant;

fn flip_last_1(mut x: u128) -> u128 {
    x &= x - 1;
    x
}

fn flip_last_one_to_zero(n: u128) -> u128 {
    n & (n-1)
}

fn main() {
    let mut x = u128::max_value();
    let now = Instant::now();
    for _ in 0..1_000{
        while x > 0 {
            x = flip_last_1(x);
        }
    }
    println!("Elapsed time: {:?}", now.elapsed());

    let now = Instant::now();
    for _ in 0..1_000{
        while x > 0 {
            x = flip_last_one_to_zero(x);
        }
    }
    println!("Elapsed time: {:?}", now.elapsed());
}
