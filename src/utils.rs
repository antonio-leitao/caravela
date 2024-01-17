use rand::seq::SliceRandom;

pub fn generate_random_sequence(n: usize) -> Vec<usize> {
    assert!(n < 120, "n must be smaller than 120");

    let mut rng = rand::thread_rng();
    let mut sequence: Vec<usize> = (0..120).collect();
    sequence.shuffle(&mut rng);

    sequence.truncate(n);
    sequence.sort();
    sequence
}
