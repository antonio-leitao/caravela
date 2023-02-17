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

pub fn random_permutations(n: usize, k: usize) -> Vec<Vec<usize>> {
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
pub fn pack_vector_to_u128(vector: &[usize]) -> u128 {
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
