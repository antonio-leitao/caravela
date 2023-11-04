use rand::Rng;

fn generate_random_matrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen_range(-10.0..10.0)).collect())
        .collect()
}

// pub fn random_permutations(n: usize, k: usize) -> Vec<Vec<usize>> {
//     let mut rng = thread_rng();
//     let mut result = vec![];
//
//     for _ in 0..n {
//         let mut permutation = (0..k).collect::<Vec<_>>();
//         permutation.shuffle(&mut rng);
//         result.push(permutation);
//     }
//     result
// }
