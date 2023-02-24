use std::hash::BuildHasher;
use std::collections::HashMap;
use std::time::Instant;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn apply_mask_fast(value: u128, mask: u128) -> u64 {
    let mut result: u128 = 0;
    let mut bit_index: u8 = 0;

    let mut m:u128 = mask;
    while m != 0 {
        let bit = m.trailing_zeros() as u8;
        m &= m - 1; // unset the lowest set bit
        result |= ((value >> bit) & 1) << bit_index;
        bit_index += 1;
    }
    result as u64
}

fn random_u128_mask(num_ones: u32) -> u128 {
    assert!(num_ones == 8 || num_ones == 16 || num_ones == 32 || num_ones == 64);

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


pub struct MaskedHasher {
    mask: u128,
    state:u64,
}

impl MaskedHasher {
    pub fn new(num_ones:u32 ) -> Self {
       let mask:u128 = random_u128_mask(num_ones);
        MaskedHasher { mask,state:0 }
    }
}

impl std::hash::Hasher for MaskedHasher {
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use of NoHashHasher")
    }
    fn write_u128(&mut self, value: u128) {
        self.state = apply_mask_fast(value, self.mask)  as u64;
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

impl BuildHasher for MaskedHasher {
    type Hasher = MaskedHasher;

    fn build_hasher(&self) -> Self::Hasher {
        MaskedHasher {
            mask: self.mask,
            state: 0,
        }
    }
}


#[test]
fn test_masked_hasher() {
    let mut map = HashMap::with_hasher(MaskedHasher::new(32));
    map.insert(0b1010_1010_1010_1010, "value1");
    map.insert(0b0101_0101_0101_0101, "value2");
    map.insert(0b1111_0000_1111_0000, "value3");

    let key:u128 = 0b1010_1010_1010_1010;
    let value = map.get(&key);
    assert_eq!(value, Some(&"value1"));
}

#[test]
fn test_correct_number_of_bits() {
    let num_ones = 8;
    let random_binary = random_u128_mask(num_ones);
    let count_ones = random_binary.count_ones();
    assert_eq!(count_ones, num_ones);
}

#[test]
fn test_random_bit_selection() {
    let num_ones = 8;
    let first_random = random_u128_mask(num_ones);
    let second_random = random_u128_mask(num_ones);
    assert_ne!(first_random, second_random, "Bit selection is not random");
}

#[test]
fn test_hasher_time(){
    let mask:u128 = random_u128_mask(32);
    let mut distilled :u64=0;
    let t0 = Instant::now();
    for i in 0..1_000_000{
        distilled+=apply_mask_fast(i, mask)  as u64;
    }
    let elapsed = t0.elapsed().as_secs_f64();
    assert!(elapsed<0.25);
    assert!(distilled>1);

}


