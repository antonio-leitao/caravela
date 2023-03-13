pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    let mut sum:f64 = 0.0;
    for (a_i, b_i) in a.iter().zip(b) {
        let diff = a_i - b_i;
        sum += diff * diff;
    }
    sum.sqrt()
}

fn hamming_distance(v1: u128, v2: u128,weights:u128) -> u32 {
    ((v1^v2)^weights).count_ones()
}

