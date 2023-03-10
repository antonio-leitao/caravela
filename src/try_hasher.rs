use rand::seq::SliceRandom;
use rand::thread_rng;

fn random_u128_mask(num_ones: usize) -> u128 {
    let mut rng = thread_rng();
    let mut indices: Vec<usize> = (0..128 as usize).collect();
    indices.shuffle(&mut rng);
    indices.truncate(num_ones as usize);

    let mut bits: u128 = 0;
    for i in indices {
        bits |= 1 << i;
    }
    bits
}

// fn masked_hash(mask: u128, input: u128) -> u64 {
//     let masked_input = mask & input;
//     let h1: u64 = masked_input as u64;
//     let h2: u64 = (masked_input >> 64) as u64;
//     h1 ^ h2
// }

pub struct IndexHasher<T>{
    mask: u128,
    map: hashbrown::HashMap<u128, T>
}

impl<T> IndexHasher<T> {
    pub fn new(n_bits: usize) -> IndexHasher<T> {
        IndexHasher {
            mask : random_u128_mask(n_bits),
            map: hashbrown::HashMap::new()
        }
    }

    pub fn insert(&mut self, input: u128, value: T) {
        let key = input ^ self.mask;
        self.map.insert(key,value);
    }

    pub fn get(&self, input: u128) -> Option<&T> {
        let key = input ^ self.mask;
        self.map.get(&key)
    }
    
   
}
