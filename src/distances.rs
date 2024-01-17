use rblas::{Dot, Nrm2};

pub fn cosine_distance(point: &[f64], simplex: &[Vec<f64>]) -> Vec<f64> {
    // Assume that simplex is normed
    let norm = Nrm2::nrm2(point);
    let distances: Vec<f64> = simplex
        .iter()
        .map(|row| 1.0 - Dot::dot(row, point) / norm)
        .collect();
    distances
}

pub fn distances_to_u128(vector: &[f64]) -> u128 {
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
