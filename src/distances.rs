pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    let mut sum:f64 = 0.0;
    for (a_i, b_i) in a.iter().zip(b) {
        let diff = a_i - b_i;
        sum += diff * diff;
    }
    sum.sqrt()
}
