mod hasher;
mod indexes;
// use rand::seq::SliceRandom;
// use rand::thread_rng;

// fn random_binary_with_ones(num_ones: u32, num_bits: u32) -> u64 {
//     assert!(num_bits == 8 || num_bits == 16 || num_bits == 32 || num_bits == 64);
//     assert!(num_ones <= num_bits);

//     let mut rng = thread_rng();
//     let mut indices: Vec<usize> = (0..num_bits as usize).collect();
//     indices.shuffle(&mut rng);
//     indices.truncate(num_ones as usize);

//     let mut bits: u64 = 0;
//     for i in indices {
//         bits |= 1 << i;
//     }
//     bits
// }




fn main(){
    println!("this is main")
}