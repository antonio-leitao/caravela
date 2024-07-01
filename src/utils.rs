pub fn argsort(data: &[f64]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_unstable_by(|&i, &j| data[i].total_cmp(&data[j]));
    indices
}
