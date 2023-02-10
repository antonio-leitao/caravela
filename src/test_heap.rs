use std::collections::{BinaryHeap, BTreeSet};
use std::time::Instant;

const N: usize = 1_000_000;

fn main() {
    let mut binary_heap = BinaryHeap::with_capacity(N);
    let mut btree_set = BTreeSet::new();

    let start = Instant::now();

    for i in 0..N {
        binary_heap.push(i);
    }

    let binary_heap_elapsed = start.elapsed();

    let start = Instant::now();

    for i in 0..N {
        btree_set.insert(i);
    }

    let btree_set_elapsed = start.elapsed();

    println!("BinaryHeap elapsed: {:?}", binary_heap_elapsed);
    println!("BTreeSet elapsed: {:?}", btree_set_elapsed);
}