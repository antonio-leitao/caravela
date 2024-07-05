use pyo3::prelude::*;
use rayon::prelude::*;
mod encode;
mod insert;
mod linalg;
mod query;
mod sphere_codes;
use insert::Insert;
use query::Query;

const NUM_PIVOTS: usize = 64;
const NUM_CHUNKS: usize = 8;
const MIN_BITS: u32 = 14 * 8;
type EncodedPoint = [u64; NUM_CHUNKS];
//###################### PYTHON INTERFACE ##########################
//Anything inside this section is exposed to python
#[pymodule]
fn caravela(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(count_sequences, m)?)?;
    m.add_class::<Caravela>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[pyclass(name = "Caravela")]
struct Caravela {
    pivots: Vec<Vec<Vec<f32>>>, //anchors
    codex: Vec<EncodedPoint>,
    data: Vec<Vec<f32>>,
}

#[pymethods]
impl Caravela {
    #[new]
    #[pyo3(signature = (n_dims))]
    fn new(n_dims: usize) -> Self {
        let mut pivots = Vec::new();
        let chunk_size = n_dims / NUM_CHUNKS;
        let mut remainder = n_dims;
        while remainder >= chunk_size {
            pivots.push(sphere_codes::generate(NUM_PIVOTS, chunk_size));
            remainder -= chunk_size;
        }
        if remainder > 0 {
            pivots.push(sphere_codes::generate(NUM_PIVOTS, remainder));
        }
        //start the dude
        Caravela {
            pivots,
            codex: Vec::new(),
            data: Vec::new(),
        }
    }
    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<f32>>) -> PyResult<()> {
        for point in data {
            self._insert(point);
        }
        Ok(())
    }

    #[pyo3(signature = (data,n_neighbors))]
    fn query(&mut self, data: Vec<Vec<f32>>, n_neighbors: usize) -> PyResult<Vec<Vec<usize>>> {
        let labels: Vec<Vec<usize>> = data
            .into_par_iter()
            .map(|point| self._single_query(point, n_neighbors))
            .collect();
        Ok(labels)
    }
}
