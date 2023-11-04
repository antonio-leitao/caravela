use ndarray::{Array2,Array1};
use pyo3::prelude::*;
mod anchors;
mod distances;
mod hashers;
mod utils;
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
    nodes: Vec<hashers::MainHash>,
    simplex: Option<Array2<f64>>, //it is option because it might not be fit
    anchors: Vec<usize>,
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
        }
    }
    #[pyo3(signature = (data))]
    fn fit(&mut self, data: Vec<Vec<f64>>) -> PyResult<()> {
        self._fit(data);
        Ok(())
    }
    #[pyo3(signature = (data,n_neighbors))]
    fn query(&mut self, data: Vec<Vec<f64>>, n_neighbors: usize) -> PyResult<Vec<Vec<usize>>> {
        println!("Running batch query");
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
        let cols = data[0].len();
        let eigen = anchors::pca_simplex(&data);
        let simplex = Array2::from_shape_vec(
            (16, cols), eigen.into_iter().flatten().collect()
        ).expect("Failed to Create shape vector");

        let binaries:Vec<u128> = data
            .into_iter()
            .map(|point| {
                let point_arr = Array1::from_vec(point);
                let dist = distances::euclidean_distance(point_arr, &simplex);
                distances::distances_to_u128(dist)
            }).collect();

        self.simplex = Some(simplex);
        self._set_nodes(binaries);
    }

    fn _set_nodes(&mut self, binaries: Vec<u128>) {
        for n_bits in self.anchors.iter() {
            let mut node = hashers::MainHash::new(*n_bits);
            for (value, key) in binaries.iter().enumerate() {
                node.insert(*key,value);  // Assuming you want to use the index as the value
            }
            self.nodes.push(node);
        }
   }
    fn _single_query(&self, point: Vec<f64>, n_neighbors: usize) -> Vec<usize> {
        println!("Computing the distance of each anchor"); //distance.rs
        println!("Converting each point into a u128"); // utils.rs
                                                       // different block
        println!("Deciding which hashmaps to visit (maybe all)"); // (here/query)
        println!("Getting candidates (counts vs approx distance)"); // (here/query)
        println!("Ordering candidates (BTree?)"); //
        println!("Return ordered candidates");
        vec![2 as usize]
    }
    fn _insert(&self, point: Vec<f64>) {
        println!("Computing the distance of each anchor");
        println!("Converting each point into a u128");
        println!("Adding point to hashmaps");
    }
}
