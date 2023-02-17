use nohash_hasher::BuildNoHashHasher;
use rand::Rng;
use std::collections::HashMap;

// fn select_32_bits() -> impl Fn(u128) -> u32 {
//     let mut rng = rand::thread_rng();
//     let shift = rng.gen_range(0..97);
//     let mask = 0xFFFF_FFFFu128 << shift;
//     move |v| ((v & mask) >> shift) as u32
// }

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
    fn hash(&self, _: u128) -> u32;
}

struct Select32Bits;

impl HashFn for Select32Bits {
    fn hash(&self, v: u128) -> u32 {
        let mut rng = rand::thread_rng();
        let shift = rng.gen_range(0..97);
        let mask = 0xFFFF_FFFFu128 << shift;
        ((v & mask) >> shift) as u32
    }
}

pub struct Index32 {
    maps: Vec<HashMap<u32, Vec<usize>, BuildNoHashHasher<u32>>>,
    functions: Vec<Box<Select32Bits>>,
    n_hashes: usize,
}

impl Index32 {
    pub fn new(n_hashes: usize) -> Self {
        let mut functions = Vec::new();
        let mut maps = Vec::new();
        for _ in 0..n_hashes {
            functions.push(Box::new(Select32Bits));
            let hasher: HashMap<u32, Vec<usize>, BuildNoHashHasher<u32>> =
                HashMap::with_hasher(BuildNoHashHasher::default());
            maps.push(hasher);
        }
        let index = Index32 {
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
