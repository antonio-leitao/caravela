pub fn euclidean(a: &[f32], b: &[f32]) -> f32 {
    let mut sum: f32 = 0.0;
    for (a_i, b_i) in a.iter().zip(b) {
        let diff = a_i - b_i;
        sum += diff * diff;
    }
    sum.sqrt()
}

//revert distances
fn revert_ij(k: i32, n: i32) -> (usize, usize) {
    let sqrt_val = f32::sqrt((-8 * k + 4 * n * (n - 1) - 7) as f32);
    let i_float = (sqrt_val / 2.0) - 0.5;
    let i = n - 2 - i_float.floor() as i32;
    let j = k + i + 1 - ((n * (n - 1)) / 2) + (((n - i) * ((n - i) - 1)) / 2);
    (i as usize, j as usize)
}

pub fn heuristic_euclidean(v1: u128, v2: u128, weights: &[f32]) -> f32 {
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

pub fn hamming(v1: u128, v2: u128) -> u32 {
    (v1 ^ v2).count_ones()
}
