mod utils;
use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;

const N:usize = 20000;
const K:usize=16;

fn pack_vector_to_u128(vector: &[usize]) -> u128 {
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

fn main(){
    let permutations = utils::random_permutations(N,K);
    println!("{:?}",permutations[0]);
    let mut hm = HashMap::with_hasher(BuildNoHashHasher::<usize>::default());
    for permutation in permutations{
        hm.insert(pack_vector_to_u128(&permutation) as usize,permutation);
    }
}