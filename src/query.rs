use crate::linalg;
use crate::Caravela;
use crate::Encode;
use nohash_hasher::{IntMap, IntSet};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

fn query_permutations(
    arr: &[u32],
    map: &IntMap<u32, Vec<usize>>,
    x: u32,
    target_sum: usize,
) -> (Vec<u32>, IntSet<usize>) {
    if arr.is_empty() {
        return (vec![], IntSet::default());
    }

    let pos = match arr.binary_search(&x) {
        Ok(index) => index,
        Err(index) => index,
    };

    let mut left = if pos == 0 { 0 } else { pos - 1 };
    let mut right = pos;
    let mut result = Vec::new();
    let mut accumulated_sum = 0;
    let mut flattened = IntSet::default();

    while accumulated_sum < target_sum && (left < arr.len() || right < arr.len()) {
        if left < arr.len()
            && (right >= arr.len() || x.abs_diff(arr[left]) <= arr[right].abs_diff(x))
        {
            let key = arr[left];
            if let Some(vec) = map.get(&key) {
                accumulated_sum += vec.len();
                result.push(key);
                flattened.extend(vec.iter().cloned());
            }
            if left > 0 {
                left -= 1;
            } else {
                left = arr.len();
            }
        } else {
            let key = arr[right];
            if let Some(vec) = map.get(&key) {
                accumulated_sum += vec.len();
                result.push(key);
                flattened.extend(vec.iter().cloned());
            }
            if right < arr.len() - 1 {
                right += 1;
            } else {
                right = arr.len();
            }
        }
    }

    (result, flattened)
}

#[derive(Copy, Clone, Debug)]
struct PointDistance {
    index: usize,
    distance: f32,
}

impl Eq for PointDistance {}

impl PartialEq for PointDistance {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl Ord for PointDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.total_cmp(&self.distance)
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
    indices: &IntSet<usize>,
    k: usize,
) -> Vec<usize> {
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for &index in indices {
        let dist = linalg::euclidean_squared(query, &points[index]);
        let point_dist = PointDistance {
            index,
            distance: dist,
        };
        if heap.len() < k {
            heap.push(Reverse(point_dist));
        } else if dist < heap.peek().unwrap().0.distance {
            heap.pop();
            heap.push(Reverse(point_dist));
        }
    }
    heap.into_iter().map(|Reverse(pd)| pd.index).collect()
}

pub trait Query {
    fn _single_query(&self, point: Vec<f32>, n_neighbors: usize, slack_factor: f64) -> Vec<usize>;
}

impl Query for Caravela {
    fn _single_query(&self, point: Vec<f32>, n_neighbors: usize, slack_factor: f64) -> Vec<usize> {
        let rank = self._encode(&point);
        let n_candidates = 10.min((slack_factor * n_neighbors as f64).ceil() as usize);
        //this stops at total elements NOT UNIQUE ONES (easy to change that)
        let (_, candidates) =
            query_permutations(&self.rank_array, &self.index_map, rank, n_candidates);
        find_k_nearest(&point, &self.data, &candidates, n_neighbors)
    }
}
