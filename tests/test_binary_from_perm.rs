use std::time::Instant;
mod utils;

const K: usize = 16;
const N: usize = 1_000_000;

fn create_matrix_mask(values: &[usize]) -> u128 {
    let mut mask: u128 = 0;
    let n = K;
    let first_part = (n * (n - 1)) / 2;
    for i in 0..n {
        let second_part = ((n - i) * (n - i - 1)) / 2;
        for j in (i + 1)..n {
            if values[j] > values[i] {
                let index = first_part - second_part + (j - i - 1);
                mask |= 1 << index;
            }
        }
    }
    mask
}
//receives a permutation array and turns it into a u128 vector
fn pack_vector_to_u128(vector: &[usize]) -> u128 {
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
//creates lower triangular
fn upper_triangular(vector: &[usize]) -> u128 {
    let mut flat: u128 = 0;
    let n = vector.len();
    for i in 0..n {
        let first_part = (2 * n - 3 - i) * i / 2;
        for j in (i + 1)..n {
            if vector[i] > vector[j] {
                flat |= 1 << (first_part + j - 1)
            }
        }
    }
    flat
}

fn main() {
    let permutations = utils::random_permutations(N, K);
    println!("{:?}", permutations[0]);

    let start = Instant::now();
    for permutation in &permutations {
        _ = create_matrix_mask(permutation)
    }
    println!("Upper Triangular Mask: {:?}", start.elapsed());

    let start = Instant::now();
    for permutation in &permutations {
        _ = upper_triangular(permutation)
    }
    println!("Upper Optimized Triangular Mask: {:?}", start.elapsed());

    let start = Instant::now();
    for permutation in &permutations {
        _ = pack_vector_to_u128(permutation)
    }
    println!("Bitwise: {:?}", start.elapsed());
}
