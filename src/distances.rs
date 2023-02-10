fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    let mut sum:f64 = 0.0;
    for (a_i, b_i) in a.iter().zip(b) {
        let diff = a_i - b_i;
        sum += diff * diff;
    }
    sum.sqrt()
}

fn is_transposition(a: u128, b: u128) -> bool {
    let xor = a ^ b;
    xor.count_ones() == 1
}

fn weighted_hamming(weights: &[f32], mask: u128) -> f32 {
    let mut sum = 0.0;
    let mut count = 0;

    for i in 0..128 {
        if mask & (1 << i) != 0 {
            sum += weights[i];
            count += 1;
        }
    }

    sum / count as f32
}
