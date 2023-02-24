use crate::hasher::MaskedHasher;
use std::collections::HashMap;

pub struct Index {
    maps: Vec<HashMap<u128, Vec<usize>, MaskedHasher>>,
    hash_sequence: Vec<u32>,
}

impl Index {
    pub fn new(hash_sequence: Vec<u32>) -> Self {
        //sort the hashsequence!
        let mut maps = Vec::new();
        for num_bits in &hash_sequence {
            maps.push(HashMap::with_hasher(MaskedHasher::new(*num_bits)));
        }
        let index = Index {
            maps: maps,
            hash_sequence: hash_sequence,
        };
        index
    }
    pub fn insert(&mut self, permutation: u128, data_point: usize) {
        //check if permutation exists
        for map in self.maps.iter_mut() {
            match map.get_mut(&permutation) {
                Some(data_points) => {
                    //if it does add data point to it
                    data_points.push(data_point);
                }
                None => {
                    //else add permutation
                    map.insert(permutation, vec![data_point]);
                }
            }
        }
    }

    pub fn query(&mut self, permutation: u128) -> hashbrown::HashSet<usize> {
        let mut candidates: hashbrown::HashSet<usize> = hashbrown::HashSet::new();
        for map in self.maps.iter_mut() {
            match map.get(&permutation) {
                Some(data_points) => {
                    for point in data_points {
                        candidates.insert(*point);
                    }
                }
                None => {
                    //else add permutation
                    continue;
                }
            }
        }
        candidates
    }
}
