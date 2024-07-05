use crate::encode::Encode;
use crate::linalg;
use crate::Caravela;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::MIN_BITS;

#[derive(Copy, Clone, Debug)]
struct PointDistance {
    index: usize,
    distance: f32,
}

impl Eq for PointDistance {}

impl PartialEq for PointDistance {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Ord for PointDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}

impl PartialOrd for PointDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_k_nearest(
    query: &[f32],
    points: &[Vec<f32>],
    indices: &Vec<usize>,
    k: usize,
) -> Vec<usize> {
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for &index in indices {
        let dist = linalg::dot(query, &points[index]);
        let point_dist = PointDistance {
            index,
            distance: dist,
        };
        if heap.len() < k {
            heap.push(point_dist);
        } else if dist > heap.peek().unwrap().distance {
            heap.pop();
            heap.push(point_dist);
        }
    }
    //heap.into_iter().map(|Reverse(pd)| pd.index).collect() //unsorted
    heap.into_sorted_vec().iter().map(|pd| pd.index).collect()
}

// fn brute_find_k_nearest(query: &[f32], points: &[Vec<f32>], k: usize) -> Vec<usize> {
//     let mut heap = BinaryHeap::with_capacity(k + 1);
//     for (index, point) in points.iter().enumerate() {
//         let dist = 1.0 - linalg::dot(query, &point);
//         let point_dist = PointDistance {
//             index,
//             distance: dist,
//         };
//         if heap.len() < k {
//             heap.push(Reverse(point_dist));
//         } else if dist < heap.peek().unwrap().0.distance {
//             heap.pop();
//             heap.push(Reverse(point_dist));
//         }
//     }
//     //heap.into_iter().map(|Reverse(pd)| pd.index).collect() //unsorted
//     heap.into_sorted_vec()
//         .iter()
//         .map(|Reverse(pd)| pd.index)
//         .collect()
// }

#[inline(always)]
fn hamming_distance(x: &[u64; 8], y: &[u64; 8]) -> u32 {
    let diff1 = x[0] ^ y[0];
    let diff2 = x[1] ^ y[1];
    let diff3 = x[2] ^ y[2];
    let diff4 = x[3] ^ y[3];
    let diff5 = x[4] ^ y[4];
    let diff6 = x[5] ^ y[5];
    let diff7 = x[6] ^ y[6];
    let diff8 = x[7] ^ y[7];
    diff1.count_ones()
        + diff2.count_ones()
        + diff3.count_ones()
        + diff4.count_ones()
        + diff5.count_ones()
        + diff6.count_ones()
        + diff7.count_ones()
        + diff8.count_ones()
}

pub trait Query {
    fn _single_query(&self, point: Vec<f32>, n_neighbors: usize) -> Vec<usize>;
}

impl Query for Caravela {
    fn _single_query(&self, point: Vec<f32>, n_neighbors: usize) -> Vec<usize> {
        let query = self._encode(&point);
        let candidates = self
            .codex
            .iter()
            .enumerate()
            .filter_map(|(index, code)| {
                if hamming_distance(code, &query) < MIN_BITS {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        find_k_nearest(&point, &self.data, &candidates, n_neighbors)
        // brute_find_k_nearest(&point, &self.data, n_neighbors)
    }
}
