
mod utils;

fn permutation_to_u128(vector: &[usize]) -> u128 {
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

fn is_transposition(a: u128, b: u128) -> bool {
    let xor = a ^ b;
    xor.count_ones() == 1
}

fn all_transpositions(mut n: u128) -> impl Iterator<Item = u128> {
    let mut mask = 1;
    let mut flips = Vec::new();
    while n > 0 {
        flips.push(n ^ mask);
        mask <<= 1;
        n >>= 1;
    }
    flips.into_iter()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Node {
    id: u128,
    data_points: Vec<usize>,
    neighbours: Vec<usize>,
}

struct Index {
    nodes: Vec<Node>,
    index_map: hashbrown::HashMap<u128, usize>,
}

impl Index {
    fn new() -> Self {
        let mut index = Index {
            nodes: Vec::new(),
            index_map: hashbrown::HashMap::new(),
        };

        //indclude origin at start
        index.add_permutation(Node {
            id: 0,
            data_points:Vec::new(),
            neighbours: Vec::new(),
        });

        index
    }

    fn add_permutation(&mut self, node: Node) {
        let id = node.id;
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        self.index_map.insert(id, index);
    }

    fn add_new_permutation(&mut self, permutation: u128, data_point: usize) {
        let mut neighbours: Vec<usize> = Vec::new();
        let permutation_index = self.nodes.len() - 1;
        // for node in &mut self.nodes{
        //     if !is_transposition(permutation, node.id){
        //         continue
        //     }
        //     node.neighbours.push(permutation_index);
        //     let index = self.index_map.get(&node.id).unwrap();
        //     neighbours.push(*index);
            
        // }

        for transposition in all_transpositions(permutation) {
            match self.index_map.get(&transposition) {
                Some(index) => {
                    //if exists add indexes to both
                    self.nodes[*index].neighbours.push(permutation_index);
                    neighbours.push(*index);
                }
                None => continue,
            }
        }
        self.add_permutation(Node {
            id: permutation,
            data_points:vec![data_point],
            neighbours: neighbours,
        });
    }


    fn insert(&mut self, permutation:u128, data_point: usize) {
        //check if permutation exists
        match self.index_map.get(&permutation) {
            Some(index) => {
                //if it does add data point to it
                self.nodes[*index].data_points.push(data_point);
            }
            None => {
                //else add permutation
                self.add_new_permutation(permutation, data_point);
            }
        }
    }
}

use std::time::Instant;


fn main(){
    let mut index:Index = Index::new();
    const N:usize = 200000;
    const K:usize =16;
    let permutations = utils::random_permutations(N,K);
    let start = Instant::now();
    for (data_point,permutation) in permutations.iter().enumerate(){
       index.insert(permutation_to_u128(&permutation),data_point);
    }
    println!("Version 1: {:?}", start.elapsed());
    println!("Index has: {:?} entries", index.nodes.len());
    println!("{:?}", index.nodes[0]);
    let mut count=0;
    for node in index.nodes{
        if node.neighbours.len()>0{
            count+=1
        }
    }
    println!("number of elemnts with neighbours {}", count)
}
