// NDARRAY
// pub fn pca_simplex(data: Vec<Vec<f64>>, k: usize) -> VecVec<<f64>> {
//     // calculate the truncated singular value decomposition for 2 singular values
//     let result = TruncatedSvd::new(data, TruncatedOrder::Largest)
//         .decompose(k)
//         .unwrap();
//     // acquire singular values, left-singular vectors and right-singular vectors
//     let (_, _, v_t) = result.values_vectors();
//     v_t
// }

extern crate nalgebra as na;

pub fn pca_simplex(data: &Vec<Vec<f64>>) -> Vec<Vec<f64>>{
    // Convert input data to nalgebra matrix
    // let matrix = na::DMatrix::from_vec(data.len(), data[0].len(), data.clone().into_iter().flatten().collect());
    let matrix = na::DMatrix::from_vec(
        data.len(), data[0].len(), data.iter().flat_map(|v| v).cloned().collect());
    // Compute the Singular Value Decomposition
    let eigenvectors = matrix.svd(false, true).v_t.expect("Error Calculating the eigenvectors");
    // Extract the first 16 columns of V^T as eigenvectors
    let anchors: Vec<Vec<f64>> =eigenvectors
        .fixed_rows::<16>(0)
        .row_iter()
        .map(|row| row.iter().cloned().collect())
        .collect();
    anchors
}

