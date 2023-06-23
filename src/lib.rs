// Here goes the Python interface
// mask_hasher.rs -> hashmap with hasing step
// create simplex -> PCA and simplex
// distance vector ->receives vector. Calls distance function to simplex each simplex ouputs vector of 16 distances
// approx_dist -> receives two binaries and a distance_vec XOR's them (euclideates where they are different) returns float

//receives a permutation array and turns it into a u128 vector

pub fn distances_to_u128(vector: &[f32]) -> u128 {
    let mut flat: u128 = 0;
    let n = vector.len();
    for i in 0..n {
        let first_part = (2 * n - 3 - i) * i / 2;
        for j in (i + 1)..n {
            if vector[i] > vector[j] {
                flat |= 1 << (first_part + j - 1)
            }
        }
    }
    flat
}


