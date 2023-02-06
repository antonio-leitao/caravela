use std::collections::HashMap;
use std::time::Instant;

const ARRAY_SIZE: usize = 16;
const MAP_SIZE: usize = 16;

fn main() {
    // Create an array of Option<u8> values
    let mut array = [None; ARRAY_SIZE];
    for i in 0..ARRAY_SIZE {
        array[i] = Some(i as u8);
    }

    // Create a hashmap of u8 values
    let mut map = HashMap::new();
    for i in 0..MAP_SIZE {
        map.insert(i, i as u8);
    }

    // Measure the time it takes to access an element in the array and unwrap the Option value
    let start = Instant::now();
    for i in 0..1_000_000 {
        let index = i % ARRAY_SIZE;
        let _ = array[index].unwrap();
    }
    let elapsed = start.elapsed();
    println!("Accessing elements in array and unwrapping Option: {:?}", elapsed);

    // Measure the time it takes to retrieve the keys from the hashmap and index it
    let start = Instant::now();
    let keys: Vec<usize> = map.keys().cloned().collect();
    for i in 0..1_000_000 {
        let index = i % MAP_SIZE;
        let key = keys[index];
        let _ = map[&key];
    }
    let elapsed = start.elapsed();
    println!("Retrieving keys from hashmap and indexing it: {:?}", elapsed);

    // Measure the time it takes to access an element in the hashmap
    let start = Instant::now();
    for i in 0..1_000_000 {
        let index = i % MAP_SIZE;
        let _ = map[&index];
    }
    let elapsed = start.elapsed();
    println!("Accessing elements in hashmap: {:?}", elapsed);
}
