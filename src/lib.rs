extern crate ndarray;
extern crate ndarray_linalg;
use hashbrown::HashMap;
use ndarray::*;
use ndarray_linalg::*;
use pyo3::prelude::*;
mod mask_hasher;
//###################### PYTHON INTERFACE ##########################
//Anything inside this section is exposed to python
#[pymodule]
fn vlmc(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(count_sequences, m)?)?;
    m.add_class::<Caravela>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}

#[derive(FromPyObject, Debug, Clone)]
enum Query {
    Single(Vec<f64>),
    Batch(Vec<Vec<f64>>),
}
//--------------------------- MAIN CLASS ----------------------
#[pyclass(name = "Caravela")]
struct Caravela {
    nodes: HashMap<Vec<usize>, usize>,
    simplex: Option<Array2<f64>>, //it is option because it might not be fit
    anchors: usize,
    size_array: Vec<usize>,
}

#[pymethods]
impl Caravela {
    #[new]
    #[pyo3(signature = (size_array))]
    fn new(size_array: Vec<usize>) -> Self {
        Caravela {
            nodes: HashMap::new(),
            simplex: None,
            anchors: 16,
            size_array,
        }
    }

    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<f64>>) -> PyResult<()> {
        self._fit(data);
        Ok(())
    }

    #[pyo3(signature = (data))]
    fn transform(&self, data: Query) -> PyResult<()> {
        match (data, self.simplex.as_ref()) {
            (Query::Single(point), Some(_)) => {
                self.single_query(point);
            }
            (Query::Batch(points), Some(_)) => {
                self.batch_query(points);
            }
            (_, None) => {
                println!("Please run the fit method first")
            }
        }
        Ok(())
    }
}

//############################# BACKEND ##########################
//Here should be methods and stuff that will not be exposed to Python
impl Caravela {
    fn _fit(&mut self, data: Vec<Vec<f64>>) {
        //create simplex
        let data = create_array(data);
        self.simplex = Some(create_simplex(data, self.anchors));
        //create the index
        let mut mask_hashers: Vec<mask_hasher::MaskHash<usize>> = Vec::new();
        for &n_bits in &self.size_array {
            let mask_hasher = mask_hasher::MaskHash::new(n_bits);
            mask_hashers.push(mask_hasher);
        }
        //TODO add element to the simplex
    }
    fn insert(&mut self, point: Array1<f32>) {//inser point into index}

    fn single_query(&self, point: Vec<f64>) -> Vec<usize> {
        //creates counts
        let point = arr1(&point);
        let distances = euclidean_distances(
            &point,
            self.simplex
                .as_ref()
                .expect("ERROR: trying to access simplex when None"),
        );
        let binary = distances_to_u128(&distances);
        vec![2]
    }
    fn batch_query(&self, points: Vec<Vec<f64>>) -> Vec<Vec<usize>> {
        //consider testing this with parallelization
        points
            .into_iter()
            .map(|point| self.single_query(point))
            .collect()
    }
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

fn create_simplex(data: Array2<f64>, k: usize) -> Array2<f64> {
    // calculate the truncated singular value decomposition for 2 singular values
    let result = TruncatedSvd::new(data, TruncatedOrder::Largest)
        .decompose(k)
        .unwrap();
    // acquire singular values, left-singular vectors and right-singular vectors
    let (_, _, v_t) = result.values_vectors();
    v_t
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
