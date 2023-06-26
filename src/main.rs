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

mod utils;
use ndarray::*;
use ndarray_linalg::*;

fn create_simplex(data: Array2<f64>, k: usize) -> Array2<f64> {
    // calculate the truncated singular value decomposition for 2 singular values
    let result = TruncatedSvd::new(data, TruncatedOrder::Largest)
        .decompose(k)
        .unwrap();
    // acquire singular values, left-singular vectors and right-singular vectors
    let (_, _, v_t) = result.values_vectors();
    v_t
}

fn create_array(data: Vec<Vec<f64>>) -> Array2<f64> {
    //transofrms a vec ot vecs into a array2
    // Get the dimensions of the input vector
    let n_rows = data.len();
    let n_cols = data[0].len();
    //create array from flat vec
    let flat_vec: Vec<f64> = data.into_iter().flatten().collect();
    let array: Array2<f64> = Array2::from_shape_vec((n_rows, n_cols), flat_vec).unwrap();
    array
}

fn euclidean_distances(point: &Array1<f64>, simplex: &Array2<f64>) -> Array1<f64> {
    let diff = simplex - &point.broadcast(simplex.raw_dim()).unwrap();
    let squared_dist = diff.mapv(|x| x * x);
    let sum_squared_dist: Array1<f64> = squared_dist.sum_axis(Axis(1));
    sum_squared_dist.mapv(f64::sqrt)
}

fn distances_to_u128(vector: &Array1<f64>) -> u128 {
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

fn main() {
    //init
    let n_points = 1_000;
    let dimensions = 100;
    let data = utils::generate_random_points(n_points, dimensions);
    let data = create_array(data);
    let simplex = create_simplex(data, 16);
    println!("Result of the singular value decomposition A = UΣV^T:");
    println!(" === V^T ===");
    println!("{:?}", simplex.shape());
    //query
    let point = arr1(&utils::generate_random_points(1, dimensions)[0]);
    let distances = euclidean_distances(&point, &simplex);
    println!(" === Distances ===");
    println!("{:?}", distances);
    let binary = distances_to_u128(&distances);
    println!(" === u128 ===");
    println!("{:b}", binary);
}
