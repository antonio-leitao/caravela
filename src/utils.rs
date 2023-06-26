//ALL OF THIS CAN GO TO THE TRASH!
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
