use nohash_hasher::BuildNoHashHasher;
use rand::Rng;
use std::collections::HashMap;

// fn select_16_bits() -> impl Fn(u128) -> u16 {
//     let mut rng = rand::thread_rng();
//     let shift = rng.gen_range(0..113);
//     let mask = 0xFFFF_FFFFu128 << shift;
//     move |v| ((v & mask) >> shift) as u16
// }

// fn select_8_bits() -> impl Fn(u128) -> u8 {
//     let mut rng = rand::thread_rng();
//     let shift = rng.gen_range(0..120);
//     let mask = 0xFFFF_FFFFu128 << shift;
//     move |v| ((v & mask) >> shift) as u8
// }

trait HashFn {
    fn hash(&self, _: u128) -> u16;
}

struct Select16Bits;

impl HashFn for Select16Bits {
    fn hash(&self, v: u128) -> u16 {
        let mut rng = rand::thread_rng();
        let shift = rng.gen_range(0..113);
        let mask = 0xFFFF_FFFFu128 << shift;
        ((v & mask) >> shift) as u16
    }
}

pub struct Index16 {
    maps: Vec<HashMap<u16, Vec<usize>, BuildNoHashHasher<u16>>>,
    functions: Vec<Box<Select16Bits>>,
    n_hashes: usize,
}

impl Index16 {
    pub fn new(n_hashes: usize) -> Self {
        let mut functions = Vec::new();
        let mut maps = Vec::new();
        for _ in 0..n_hashes {
            functions.push(Box::new(Select16Bits));
            let hasher: HashMap<u16, Vec<usize>, BuildNoHashHasher<u16>> =
                HashMap::with_hasher(BuildNoHashHasher::default());
            maps.push(hasher);
        }
        let index = Index16 {
            maps: maps,
            functions: functions,
            n_hashes: n_hashes,
        };
        index
    }
    pub fn insert(&mut self, permutation: u128, data_point: usize) {
        //check if permutation exists
        for i in 0..self.n_hashes {
            let hash = self.functions[i].hash(permutation);
            match self.maps[i].get_mut(&hash) {
                Some(data_points) => {
                    //if it does add data point to it
                    data_points.push(data_point);
                }
                None => {
                    //else add permutation
                    self.maps[i].insert(hash, vec![data_point]);
                }
            }
        }
    }

    pub fn query(&mut self, permutation: u128) -> hashbrown::HashSet<usize> {
        let mut candidates: hashbrown::HashSet<usize> = hashbrown::HashSet::new();
        for i in 0..self.n_hashes {
            let hash = self.functions[i].hash(permutation);
            match self.maps[i].get(&hash) {
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
