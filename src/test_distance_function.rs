use std::time::Instant;

fn get_weight_sum_v1(binary: u128, permutation: [u128; 128]) -> u128 {
    let mut sum = 0;

    for i in 0..128 {
        if (binary & (1 << i)) != 0 {
            sum += permutation[127 - i];
        }
    }

    sum
}

fn get_weight_sum_v2(binary: u128, permutation: [u128; 128]) -> u128 {
    let mut sum = 0;
    let mut binary_copy = binary;

    while binary_copy != 0 {
        let last_bit = binary_copy & !(binary_copy - 1);
        let bit_index = (last_bit.trailing_zeros()) as usize;
        sum += permutation[bit_index];
        binary_copy -= last_bit;
    }

    sum
}

fn sum_of_set_bits(n: u128, permutation: [u128; 128]) -> u128 {
    let mut sum = 0;
    let mut index = 0;
    let mut value = n;
    while value != 0 {
        if value & 1 == 1 {
            sum +=  permutation[index];
        }
        value >>= 1;
        index += 1;
    }
    sum
}

fn sum_of_set_bits_v2(n: u128, permutation: [u128; 128]) -> u128 {
    let mut sum = 0;
    let mut value = n;
    while value != 0 {
        let index = value.trailing_zeros();
        sum += permutation[index as usize];
        value ^= 1 << index;
    }
    sum
}


const N: usize = 1_000_000;

#[test]
fn main() {
    let permutation = [1u128; 128];

    let start = Instant::now();
    for _ in 0..N {
        let _ = get_weight_sum_v1(0xffff_ffff_ffff_ffff, permutation);
    }
    let duration_v1 = start.elapsed();

    let start = Instant::now();
    for _ in 0..N {
        let _ = get_weight_sum_v2(0xffff_ffff_ffff_ffff, permutation);
    }
    let duration_v2 = start.elapsed();


    let start = Instant::now();
    for _ in 0..N {
        let _ = sum_of_set_bits(0xffff_ffff_ffff_ffff, permutation);
    }
    let duration_v3 = start.elapsed();


    let start = Instant::now();
    for _ in 0..N {
        let _ = sum_of_set_bits_v2(0xffff_ffff_ffff_ffff, permutation);
    }
    let duration_v4 = start.elapsed();

    println!("Version 1: {:?}", duration_v1);
    println!("Version 2: {:?}", duration_v2);
    println!("Version 3: {:?}", duration_v3);
    println!("Version 4: {:?}", duration_v4);
}
