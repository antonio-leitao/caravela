mod utils;
use hashbrown::HashMap;
use std::time::Instant;

const N:usize = 200000;
const K:usize=16;

fn pack_vector_to_u128(vector: Vec<usize>) -> u128 {
    let mut packed: u128 = 0;
    for i in 0..vector.len() {
        for j in i + 1..vector.len() {
            packed <<= 1;
            if vector[i] > vector[j] {
                packed |= 1;
            }
        }
    }
    packed
}

fn flip_last_one_to_zero(n: u128) -> u128 {
    n & (n-1)
}

struct Node{
    data_points: Vec<usize>,
    neighbours: Vec<u128>
}

struct Graph{
    nodes:HashMap<u128, Node>
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes:HashMap::new()
        }
    }
    fn add_node(&mut self, permutation:u128) {
        if !self.nodes.contains_key(&permutation) {
            self.nodes.insert(permutation, Node{data_points:vec![],neighbours:vec![permutation]});
        }
    }
}



fn main(){
    //generates random permutations
    let permutations = utils::random_permutations(N,K);
    //let mut Graph:HashMap<usize, Node, std::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<usize>>> = HashMap::with_hasher(BuildNoHashHasher::<usize>::default());
    
    let start = Instant::now();
    let mut graph = Graph::new();
    for permutation in permutations{
        //create binary
        graph.add_node(pack_vector_to_u128(permutation))
       
        //if doesnt exist iterate through all existing keys

        // if key is a transposition -> append permutation to neighbours-> Append key to neighbours
        
    }
    println!("Graph len: {}", graph.nodes.len());
    println!("Indexing 200000 points: {:?}", start.elapsed());
}

