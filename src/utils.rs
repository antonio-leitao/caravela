use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn generate_random_points(n_points: usize, dimensions: usize) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let mut points = Vec::with_capacity(n_points);
    for _ in 0..n_points {
        let mut point = Vec::with_capacity(dimensions);
        for _ in 0..dimensions {
            point.push(rng.gen::<f64>());
        }
        points.push(point);
    }
    points
}

fn random_permutations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut rng = thread_rng();
    let mut result = vec![];

    for _ in 0..n {
        let mut permutation = (0..k).collect::<Vec<_>>();
        permutation.shuffle(&mut rng);
        result.push(permutation);
    }
    result
}

//receives a permutation array and turns it into a u128 vector
pub fn distances_to_u128(vector: &[f64]) -> u128 {
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

// fn generate_non_repeated_sequence(n:usize) -> Vec<u8> {
//     let mut rng = rand::thread_rng();
//     let mut used_numbers = HashSet::new();
//     let mut numbers:Vec<u8> = Vec::new();
//     while numbers.len() < n {
//         let number = (0..128).choose(&mut rng).unwrap();
//         if used_numbers.insert(number) {
//             numbers.push(number);
//         }
//     }
//     numbers
// }

// fn extract_bits_at_indexes(indexes: &[u8], input: u128) -> u64 {
//     let mut result: u64 = 0;
//     for (i, &index) in indexes.iter().enumerate() {
//         if input & (1u128 << index) != 0 {
//             result |= 1 << i;
//         }
//     }
//     result
// }


pub fn random_u128_mask(num_ones: usize) -> u128 {
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