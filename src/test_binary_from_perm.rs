mod utils;
mod distances;
use std::time::Instant;

const K:usize=16;
const N:usize=2000;


//receives a permutation array and turns it into a u128 vector
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

   let start = Instant::now();
   let mut binary: Vec<u8> = vec![];
   for permutation in &permutations{
    for i in 0..K {
        for j in i+1..K {
            if permutation[i] > permutation[j] {
                binary.push(0);
            } else {
                binary.push(1);
            }
        }
    }
   }
    println!("Vector: {:?}", start.elapsed());


    let start = Instant::now();
   for permutation in &permutations{
    let mut binary: [u8; 120] = [0; 120];
    let mut index:usize =0;
    for i in 0..K {
        for j in i+1..K {
            if permutation[i] > permutation[j] {
                binary[index]=0;
            } else {
                binary[index]=1;
            }
        index +=1
        }
    }
   }
    println!("Array: {:?}", start.elapsed());

    let start = Instant::now();
    for permutation in &permutations{
        let mut binary: [u8; 120] = [0; 120];
        let mut index: usize = 0;
        for i in 0..K {
            for j in i+1..K {
                binary[index] = (permutation[i] > permutation[j]) as u8;
                index += 1;
            }
        }
    }
    println!("Array optimized: {:?}", start.elapsed());


    let start = Instant::now();
    for permutation in &permutations{
        _ = pack_vector_to_u128(permutation)
    }
    println!("Bitwise: {:?}", start.elapsed());

}



