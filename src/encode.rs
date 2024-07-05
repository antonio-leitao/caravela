use crate::linalg;
use crate::{Caravela, EncodedPoint, NUM_CHUNKS};
const NUM_SELECTION: usize = 32;

fn encode_chunk(point: &[f32], pivots: &[Vec<f32>]) -> u64 {
    let mut distances: Vec<(f32, usize)> = pivots
        .iter()
        .enumerate()
        .map(|(i, pivot)| {
            let distance = 1.0 - linalg::dot(point, pivot);
            (distance, i)
        })
        .collect();
    distances.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    distances
        .iter()
        .take(NUM_SELECTION)
        .fold(0u64, |acc, &(_, index)| acc | (1u64 << index))
}

pub trait Encode {
    fn _encode(&self, point: &[f32]) -> EncodedPoint;
}

impl Encode for Caravela {
    fn _encode(&self, point: &[f32]) -> EncodedPoint {
        let mut encoded = [0u64; NUM_CHUNKS];
        let chunk_size = point.len() / NUM_CHUNKS;

        for (i, chunk_pivots) in self.pivots.iter().enumerate() {
            let start = i * chunk_size;
            let end = if i == NUM_CHUNKS - 1 {
                point.len()
            } else {
                start + chunk_size
            };
            encoded[i] = encode_chunk(&point[start..end], chunk_pivots);
        }
        encoded
    }
}
