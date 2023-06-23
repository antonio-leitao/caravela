// mod distances;
// mod lib;
// mod mask_hasher;
// mod utils;
// use std::time::Instant;
//
// fn calculate_distances(point: &[f32], simplex: &[[f32; 16]]) -> [f32; 16] {
//     let mut distance_vector: [f32; 16] = [0.0; 16];
//     for (j, vertex) in simplex.iter().enumerate() {
//         distance_vector[j] = distances::euclidean(point, vertex);
//     }
//     distance_vector
// }
//
// fn main() {
//     let n_points = 1_00_000;
//     let dimensions = 100;
//     let n_bits = 12;
//
//     let data = utils::generate_random_points(n_points, dimensions);
//     let simplex = utils::generate_random_points(16, dimensions);
//
//     let mut hm: mask_hasher::MaskHash<usize> = mask_hasher::MaskHash::new(n_bits);
//
//     let t0 = Instant::now();
//     //calculate distance to simplex
//     for (i, point) in data.iter().enumerate() {
//         let mut distance_vector: [f32; 16] = [0.0; 16];
//         for (j, vertex) in simplex.iter().enumerate() {
//             distance_vector[j] = distances::euclidean(point, vertex);
//         }
//         let binary_vector = lib::distances_to_u128(&distance_vector);
//         hm.insert(binary_vector, i)
//     }
//     println!("Index construction took: {:?}", t0.elapsed());
//
//     let t0 = Instant::now();
//     let mut sum: i64 = 0;
//     for i in 0..1_000_000u32 {
//         //let input= utils::random_u128_mask(n_bits);
//         if let Some(x) = hm.get(u128::from(i)) {
//             sum += *x as i64;
//         }
//     }
//     let elapsed = t0.elapsed().as_secs_f64();
//     println!(
//         "The sum is: {}. Query time elapsed: {:.3} sec",
//         sum, elapsed
//     );
// }
extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::*;
use ndarray_linalg::*;

fn main() {
    let a = arr2(&[[3., 2., 2.], [2., 3., -2.]]);

    // calculate the truncated singular value decomposition for 2 singular values
    let result = TruncatedSvd::new(a, TruncatedOrder::Largest)
        .decompose(2)
        .unwrap();

    // acquire singular values, left-singular vectors and right-singular vectors
    let (u, sigma, v_t) = result.values_vectors();
    println!("Result of the singular value decomposition A = UΣV^T:");
    println!(" === U ===");
    println!("{:?}", u);
    println!(" === Σ ===");
    println!("{:?}", Array2::from_diag(&sigma));
    println!(" === V^T ===");
    println!("{:?}", v_t);
}
