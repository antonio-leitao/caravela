use ndarray::*;
use ndarray_linalg::*;

pub fn create_simplex(data: Array2<f64>, k: usize) -> Array2<f64> {
    // calculate the truncated singular value decomposition for 2 singular values
    let result = TruncatedSvd::new(data, TruncatedOrder::Largest)
        .decompose(k)
        .unwrap();
    // acquire singular values, left-singular vectors and right-singular vectors
    let (_, _, v_t) = result.values_vectors();
    v_t
}
