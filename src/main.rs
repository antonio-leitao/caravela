mod try_hasher;
mod utils;
mod distances;
use std::time::Instant;



fn main(){
    let n_points = 1_00_000;
    let dimensions=100;
    let n_bits=12;

    let data = utils::generate_random_points(n_points, dimensions);
    let simplex = utils::generate_random_points(16, dimensions);
    
    let mut hm:try_hasher::IndexHasher<usize> = try_hasher::IndexHasher::new(n_bits);

    let t0 = Instant::now();
    //calculate distance to simplex
    for (i,point) in data.iter().enumerate(){
        let mut distance_vector:[f64;16]=[0.0;16];
        for (j,vertex) in simplex.iter().enumerate(){
            distance_vector[j] = distances::euclidean(point,vertex);
        }
        let binary_vector = utils::distances_to_u128(&distance_vector);
        hm.insert(binary_vector, i)
    }
    println!("Index construction took: {:?}", t0.elapsed());

    let t0 = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..1_000_000u32 {
        //let input= utils::random_u128_mask(n_bits);
        if let Some(x) = hm.get(u128::from(i)) {
            sum += *x as i64;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("The sum is: {}. Query time elapsed: {:.3} sec", sum, elapsed);

    
}
