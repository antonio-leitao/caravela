use crate::mask_hasher;

pub struct Index {
    maps: Vec<mask_hasher::MaskHash<Vec<usize>>>,
}

impl Index {
    pub fn insert(&mut self, data_point: Vec<f64>){
        
    }
}