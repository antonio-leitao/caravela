
use std::time::Instant;

fn transpose(permutation: &mut [char]) {
    let n = permutation.len();

    for i in 0..n {
        if permutation[i] != (b'A' + i as u8) as char {
            let j = permutation
                .iter()
                .position(|&x| x == (b'A' + i as u8) as char)
                .unwrap();
            permutation.swap(i, j);
            //println!("Transpose {} and {}", permutation[i], permutation[j]);
        }
    }
}

fn main() {
    let mut permutation = ['B', 'A', 'D', 'C'];
    let start = Instant::now();
    transpose(&mut permutation);
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
}
