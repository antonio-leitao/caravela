use nohash_hasher::{IntMap, IntSet};
use std::collections::hash_map::Entry;
// use rand::seq::SliceRandom;
// use rand::thread_rng;
/// In case for optimization for u64s
// fn random_u128_mask(num_ones: usize) -> u128 {
//     let mut rng = thread_rng();
//     let mut indices: Vec<usize> = (0..128 as usize).collect();
//     indices.shuffle(&mut rng);
//     indices.truncate(num_ones as usize);
//
//     let mut bits: u128 = 0;
//     for i in indices {
//         bits |= 1 << i;
//     }
//     bits
// }

fn mask_key(values: &[usize], target: u128) -> u64 {
    let mut result: u64 = 0;
    values.iter().enumerate().for_each(|(index, val)| {
        if (target & (1 << val)) != 0 {
            result |= 1 << index;
        }
    });
    result as u64
}

pub struct MaskMap {
    mask: Vec<usize>,
    hashmap: IntMap<u64, IntSet<usize>>,
}

impl MaskMap {
    pub fn new(mask: Vec<usize>) -> Self {
        MaskMap {
            mask,
            hashmap: IntMap::default(),
        }
    }
    pub fn get(&self, key: u128) -> Option<&IntSet<usize>> {
        let key: u64 = mask_key(&self.mask, key);
        self.hashmap.get(&key)
    }

    pub fn insert(&mut self, input: u128, value: usize) {
        let key: u64 = mask_key(&self.mask, input);
        match self.hashmap.entry(key) {
            Entry::Vacant(entry) => {
                let mut new_set = IntSet::default();
                new_set.insert(value);
                entry.insert(new_set);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(value);
            }
        }
    }
}
