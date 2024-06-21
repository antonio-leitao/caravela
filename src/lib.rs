use core::panic;
use nohash_hasher::IntMap;
use pyo3::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
mod distances;
mod hasher;
mod linalg;
mod utils;
use hasher::MaskMap;
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
    nodes: Vec<MaskMap>,
    simplex: Option<Vec<Vec<f64>>>, //it is option because it might not be fit
    anchors: Vec<usize>,
    n_entries: usize,
}

#[pymethods]
impl Caravela {
    #[new]
    #[pyo3(signature = (anchors))]
    fn new(anchors: Vec<usize>) -> Self {
        Caravela {
            nodes: Vec::new(),
            simplex: None,
            anchors,
            n_entries: 0,
        }
    }
    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<f64>>) -> PyResult<()> {
        self._fit(data);
        Ok(())
    }
    #[pyo3(signature = (data,n_neighbors))]
    fn query(&mut self, data: Vec<Vec<f64>>, n_neighbors: usize) -> PyResult<Vec<Vec<usize>>> {
        let labels: Vec<Vec<usize>> = data
            .into_iter()
            .map(|point| self._single_query(point, n_neighbors))
            .collect();
        Ok(labels)
    }
}

//############################# BACKEND ##########################
//Here should be methods and stuff that will not be exposed to Python
impl Caravela {
    fn _fit(&mut self, data: Vec<Vec<f64>>) {
        // can be slow
        let simplex = linalg::pca_simplex(&data);
        let binaries: Vec<u128> = data
            .into_iter()
            .map(|point| {
                let distances: Vec<f64> = simplex
                    .iter()
                    .map(|row| distances::euclidean(row, &point))
                    .collect();
                distances::distances_to_u128(&distances)
            })
            .collect();

        self.simplex = Some(simplex);
        self._set_nodes(binaries);
    }

    fn _set_nodes(&mut self, binaries: Vec<u128>) {
        for n_bits in self.anchors.iter() {
            let mask = utils::generate_random_sequence(*n_bits);
            let mut node = MaskMap::new(mask);
            for (value, key) in binaries.iter().enumerate() {
                node.insert(*key, value); // Assuming you want to use the index as the value
            }
            self.nodes.push(node);
        }
        self.n_entries = binaries.len() //will be used if the user wants to insert more
    }

    fn _single_query(&self, point: Vec<f64>, n_neighbors: usize) -> Vec<usize> {
        let simplex = match &self.simplex {
            Some(simplex) => simplex,
            None => panic!("Caravela has not been fit"), //Substitute with random index
        };
        let query = distances::get_position(&point, &simplex);

        // BLOCK HERE MAYBE SEPARATE
        let mut counts: IntMap<usize, usize> = IntMap::default();
        for mask_map in self.nodes.iter() {
            if let Some(int_set) = mask_map.get(query) {
                for &value in int_set {
                    *counts.entry(value).or_insert(0) += 1;
                }
            }
        }

        let mut heap = BinaryHeap::new();

        for (index, count) in counts.into_iter() {
            heap.push(Reverse((count, index)));
            if heap.len() > n_neighbors {
                heap.pop(); // Remove the least frequent.
            }
        }
        heap.into_sorted_vec()
            .into_iter()
            .map(|Reverse(data)| data.1)
            .collect::<Vec<_>>()
    }

    fn _insert(&mut self, point: Vec<f64>) {
        let simplex = match &self.simplex {
            Some(simplex) => simplex,
            None => panic!("Caravela has not been fit"), //Substitute with random index
        };
        let query = distances::get_position(&point, &simplex);

        self.n_entries += 1;
        for mask_map in self.nodes.iter_mut() {
            mask_map.insert(query, self.n_entries)
        }
    }
}
