
use rand::seq::SliceRandom;
use rand::thread_rng;

fn random_binary_with_ones(num_ones: u32, num_bits: u32) -> u64 {
    assert!(num_bits == 8 || num_bits == 16 || num_bits == 32 || num_bits == 64);
    assert!(num_ones <= num_bits);

    let mut rng = thread_rng();
    let mut indices: Vec<usize> = (0..num_bits as usize).collect();
    indices.truncate(num_ones as usize);
    indices.shuffle(&mut rng);

    let mut bits: u64 = 0;
    for i in indices {
        bits |= 1 << i;
    }
    bits
}

#[test]
fn test_correct_number_of_bits() {
    let num_ones = 8;
    let num_bits = 32;
    let random_binary = random_binary_with_ones(num_ones, num_bits);
    let count_ones = random_binary.count_ones();
    assert_eq!(count_ones, num_ones);
}

#[test]
fn test_random_bit_selection() {
    let num_ones = 8;
    let num_bits = 32;
    let first_random = random_binary_with_ones(num_ones, num_bits);
    let second_random = random_binary_with_ones(num_ones, num_bits);
    assert_ne!(first_random, second_random, "Bit selection is not random");
}

fn main(){
    println!("this is main")
}