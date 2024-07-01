use crate::linalg;
use crate::Caravela;

const FACTORIAL: [u32; 9] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320];

#[inline(always)]
fn rank_permutation(perm: &[usize; 8]) -> u32 {
    let mut used = 0u16;
    let mut rank = 0;

    for (i, &p) in perm.iter().enumerate() {
        let smaller = (0..p).filter(|&x| used & (1 << x) == 0).count();
        rank += (smaller as u32) * FACTORIAL[7 - i];
        used |= 1 << p;
    }

    rank
}

pub trait Encode {
    fn _encode(&self, point: &[f32]) -> u32;
}

impl Encode for Caravela {
    /// Function to find the indices of the k closest neighbors based on Hamming distance
    fn _encode(&self, point: &[f32]) -> u32 {
        let mut distances_with_indices: [(f32, usize); 16] = self
            .simplex
            .iter()
            .enumerate()
            .map(|(i, row)| (linalg::euclidean_squared(row, point), i))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        distances_with_indices
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let first_half: [usize; 8] = distances_with_indices[..8]
            .iter()
            .map(|&(_, index)| index)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        rank_permutation(&first_half)
    }
}
