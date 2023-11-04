use ndarray::{Array1, Array2, Axis};

pub fn euclidean_distance(point: Array1<f64>, simplex: &Array2<f64>) -> Array1<f64> {
    // this funciton is mega fast the collect method slows it down by 10000% no joke
    let distances: Array1<f64> = simplex
        .axis_iter(Axis(0))
        .map(|row| {
            let diff = &row - &point;
            diff.dot(&diff).sqrt()
        })
        .collect();
    distances
}

pub fn distances_to_u128(vector: Array1<f64>) -> u128 {
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

//revert distances
fn revert_ij(k: i32, n: i32) -> (usize, usize) {
    let sqrt_val = f32::sqrt((-8 * k + 4 * n * (n - 1) - 7) as f32);
    let i_float = (sqrt_val / 2.0) - 0.5;
    let i = n - 2 - i_float.floor() as i32;
    let j = k + i + 1 - ((n * (n - 1)) / 2) + (((n - i) * ((n - i) - 1)) / 2);
    (i as usize, j as usize)
}

fn heuristic_euclidean(v1: u128, v2: u128, weights: &[f32]) -> f32 {
    //given two binary vectors and a distance vector, defines a heuristic distance.
    let mut result = 0.0;
    let mut mask = v1 ^ v2;

    while mask != 0 {
        let index = mask.trailing_zeros();
        let (i, j) = revert_ij((index - 1) as i32, 16);
        let diff = weights[i] - weights[j];
        result += diff * diff;
        mask ^= 1 << index;
    }
    f32::sqrt(result)
}
