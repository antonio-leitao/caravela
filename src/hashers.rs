use nohash_hasher::{IntMap,IntSet};
use std::collections::hash_map::Entry;
use rand::seq::SliceRandom;
use rand::thread_rng;
use triple_accel::hamming;

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

pub struct DistHash<T> {
    mask: [u8; 16],
    map: IntMap<u8, T>,
}

impl<T> DistHash<T> {
    pub fn new(n_bits: usize) -> DistHash<T> {
        DistHash {
            mask: random_u128_mask(n_bits).to_le_bytes(),
            map: IntMap::default(),
        }
    }

    pub fn insert(&mut self, input: u128, value: T) {
        let input_bytes = input.to_le_bytes();
        let key = hamming(&self.mask, &input_bytes) as u8;
        self.map.insert(key, value);
    }

    pub fn get(&self, input: u128) -> Option<&T> {
        let input_bytes = input.to_le_bytes();
        let key = hamming(&self.mask, &input_bytes) as u8;
        self.map.get(&key)
    }
}

// MAIN HASHER
pub struct MainHash {
    mask: u128,
    map: hashbrown::HashMap<u128, IntSet<usize>>,
}

impl MainHash {
    pub fn new(n_bits: usize) -> MainHash {
        MainHash {
            mask: random_u128_mask(n_bits),
            map: hashbrown::HashMap::new(),
        }
    }

    pub fn insert(&mut self, input: u128, value: usize) {
        let key = input & self.mask;
        match self.map.entry(key) {
            Entry::Vacant(entry) => {
                let mut new_set = IntSet::new();
                new_set.insert(value);
                entry.insert(new_set);
            }
            Entry::Occupied(mut entry) => {
                entry.insert(value);
            }
        }
    }

    pub fn get(&self, input: u128) -> Option<&IntSet<usize>> {
        let key = input & self.mask;
        self.map.get(&key)
    }
}
// MASK HASHER
pub struct MaskHash<T> {
    mask: u128,
    map: hashbrown::HashMap<u128, T>,
}

impl<T> MaskHash<T> {
    pub fn new(n_bits: usize) -> MaskHash<T> {
        MaskHash {
            mask: random_u128_mask(n_bits),
            map: hashbrown::HashMap::new(),
        }
    }

    pub fn insert(&mut self, input: u128, value: T) {
        let key = input & self.mask;
        self.map.insert(key, value);
    }

    pub fn get(&self, input: u128) -> Option<&T> {
        let key = input & self.mask;
        self.map.get(&key)
    }
}
