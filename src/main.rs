use ndarray::*;
mod utils;
mod anchors;
mod distances;

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

fn main() {
    //init
    let n_points = 1_000;
    let dimensions = 100;
    let data = utils::generate_random_points(n_points, dimensions);
    let data = create_array(data);
    let simplex = anchors::create_simplex(data, 16);
    println!("Result of the singular value decomposition A = UΣV^T:");
    println!(" === V^T ===");
    println!("{:?}", simplex.shape());
    //query
    let point = arr1(&utils::generate_random_points(1, dimensions)[0]);
    let distances = distances::euclidean_distances(&point, &simplex);
    println!(" === Distances ===");
    println!("{:?}", distances);
    let binary = distances::distances_to_u128(&distances);
    println!(" === u128 ===");
    println!("{:b}", binary);
}
