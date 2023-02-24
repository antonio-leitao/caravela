use crate::hasher::MaskedHasher;
use std::collections::HashMap;
use crate::utils::random_permutations;


fn permutation_to_u128(vector: &[usize]) -> u128 {
    let mut packed: u128 = 0;
    for i in 0..vector.len() {
        for j in i + 1..vector.len() {
            packed <<= 1;
            if vector[i] > vector[j] {
                packed |= 1;
            }
        }
    }
    packed
}

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


#[test]
fn run_metrics(){
    let n_samples:usize = 100_000;
    let n_queries:usize = 1_000;
    
    let mut index: Index = Index::new(vec![8,16,32,64]);
    let permutations = utils::random_permutations(n_samples, 16);

    for (data_point, permutation) in permutations.iter().enumerate() {
        index.insert(permutation_to_u128(&permutation), data_point);
    }

    let queries = utils::random_permutations(n_queries, 16);
    let mut total_time = Duration::new(0, 0);
    let mut count: u32 = 0;

    for permutation in queries.iter() {
        let start_time = Instant::now();
        let candidates = index.query(permutation_to_u128(&permutation));

        let elapsed_time = start_time.elapsed();
        total_time += elapsed_time;
        count += candidates.len() as u32;
    }
    let avg_time = total_time.as_millis() as f32 / n_queries as f32;
    let avg_pool = count as f32 / n_queries as f32;
    (avg_time, avg_pool)
}