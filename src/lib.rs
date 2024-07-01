use nohash_hasher::IntMap;
use pyo3::prelude::*;
mod encode;
mod insert;
mod linalg;
mod query;
mod sphere_codes;
mod utils;
use encode::Encode;
use insert::Insert;
use query::Query;
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
    simplex: Vec<Vec<f32>>,             //anchors
    index_map: IntMap<u32, Vec<usize>>, //rank -> data
    rank_array: Vec<u32>,               //rank list
    data: Vec<Vec<f32>>,
    index: usize,
}

#[pymethods]
impl Caravela {
    #[new]
    #[pyo3(signature = (n_points,n_dims))]
    fn new(n_points: usize, n_dims: usize) -> Self {
        let simplex = sphere_codes::optimize_points(n_points, n_dims);
        Caravela {
            simplex,
            index_map: IntMap::default(),
            rank_array: Vec::new(),
            data: Vec::new(),
            index: 0,
        }
    }
    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<f32>>) -> PyResult<()> {
        self._fit(data);
        Ok(())
    }

    #[pyo3(signature = (point))]
    fn encode(&self, point: Vec<f32>) -> PyResult<u32> {
        let rank = self._encode(&point);
        Ok(rank)
    }

    #[pyo3(signature = (data,n_neighbors, slack_factor))]
    fn query(
        &mut self,
        data: Vec<Vec<f32>>,
        n_neighbors: usize,
        slack_factor: f64,
    ) -> PyResult<Vec<Vec<usize>>> {
        let labels: Vec<Vec<usize>> = data
            .into_iter()
            .map(|point| self._single_query(point, n_neighbors, slack_factor))
            .collect();
        Ok(labels)
    }
}

//############################# BACKEND ##########################
//Here should be methods and stuff that will not be exposed to Python
impl Caravela {
    fn _fit(&mut self, data: Vec<Vec<f32>>) {
        //Now add all the points no?
        for point in data {
            self._insert(point);
        }
    }
}
