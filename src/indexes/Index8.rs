use rand::Rng;
use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;

// fn select_8_bits() -> impl Fn(u128) -> u8 {
//     let mut rng = rand::thread_rng();
//     let shift = rng.gen_range(0..120);
//     let mask = 0xFFFF_FFFFu128 << shift;
//     move |v| ((v & mask) >> shift) as u8
// }


trait HashFn {
    fn hash(&self, _:u128) -> u8;
}

struct Select8Bits;

impl HashFn for Select8Bits {
    fn hash(&self, v: u128) -> u8 {
        let mut rng = rand::thread_rng();
        let shift = rng.gen_range(0..120);
        let mask = 0xFFFF_FFFFu128 << shift;
        ((v & mask) >> shift) as u8
    }
}



pub struct Index8 {
    maps: Vec<HashMap<u8, Vec<usize>, BuildNoHashHasher<u8>>>,
    functions: Vec<Box<Select8Bits>>,
    n_hashes: usize,
}

impl Index8 {
    pub fn new(n_hashes: usize) -> Self {
        let mut functions = Vec::new();
        let mut maps = Vec::new();
        for _ in 0..n_hashes {
            functions.push(Box::new(Select8Bits));
            let hasher: HashMap<u8, Vec<usize>, BuildNoHashHasher<u8>> =
                HashMap::with_hasher(BuildNoHashHasher::default());
            maps.push(hasher);
        }
        let index = Index8 {
            maps: maps,
            functions: functions,
            n_hashes: n_hashes,
        };
        index
    }
    pub fn insert(&mut self, permutation:u128, data_point: usize) {
        //check if permutation exists
        for i in 0..self.n_hashes{
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

    pub fn query(&mut self, permutation:u128) -> hashbrown::HashSet<usize>{
        let mut candidates:hashbrown::HashSet<usize> =hashbrown::HashSet::new();
        for i in 0..self.n_hashes{
            let hash = self.functions[i].hash(permutation);
            match self.maps[i].get(&hash) {
                Some(data_points) => {
                    for point in data_points{
                        candidates.insert(*point);
                    }
                }
                None => {
                    //else add permutation
                    continue
                }
            }
        }
        candidates
    }
}