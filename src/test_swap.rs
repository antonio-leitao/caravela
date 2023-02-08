use std::time::Instant;


fn swap(array: &mut [u8; 16], i: usize, j: usize) {
    let temp = array[i];
    array[i] = array[j];
    array[j] = temp;
}


fn main() {
    let mut array = [0u8; 16];
    for i in 0..16 {
        array[i] = i as u8;
    }

    let i = 3;
    let j = 12;
    let start = Instant::now();
    for _ in 1..1_000_000{
        swap(&mut array, i, j);
    }
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
}
