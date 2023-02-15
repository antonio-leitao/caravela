// mod utils;
// use hashbrown::HashMap;
// use std::time::Instant;

// const N: usize = 200000;
// const K: usize = 16;

// fn pack_vector_to_u128(vector: Vec<usize>) -> u128 {
//     let mut packed: u128 = 0;
//     for i in 0..vector.len() {
//         for j in i + 1..vector.len() {
//             packed <<= 1;
//             if vector[i] > vector[j] {
//                 packed |= 1;
//             }
//         }
//     }
//     packed
// }

// fn flip_last_one_to_zero(n: u128) -> u128 {
//     n & (n - 1)
// }

// struct Node {
//     data_points: Vec<usize>,
//     neighbours: Vec<u128>,
// }

// struct Graph {
//     nodes: HashMap<u128, Node>,
// }

// impl Graph {
//     fn new() -> Self {
//         Graph {
//             nodes: HashMap::new(),
//         }
//     }
//     fn add_node(&mut self, permutation: u128) {

//         //check if it exists
//         //iterate through permutation to -> 0
//             //iterate through all transpositions
//                 //if transposition exists add transposition as neighbour
//                 //add neighbour to list of points
//             //add permutation with list of neighbours

//         if !self.nodes.contains_key(&permutation) {
//             //if doesnt exist iterate through all transpositions
//             for i in 0..120 {
//                 binary_vector ^= 1u128 << i;
//                 //check if permutation exists
//                 //if exists add permutation as neighbour
//             }

//             self.nodes.insert(
//                 permutation,
//                 Node {
//                     data_points: vec![],
//                     neighbours: vec![permutation],
//                 },
//             );
//         //iterate thogu
//         }
//     }
// }

// fn main() {
//     //generates random permutations
//     let permutations = utils::random_permutations(N, K);
//     //let mut Graph:HashMap<usize, Node, std::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<usize>>> = HashMap::with_hasher(BuildNoHashHasher::<usize>::default());

//     let start = Instant::now();
//     let mut graph = Graph::new();
//     for permutation in permutations {
//         //create binary
//         graph.add_node(pack_vector_to_u128(permutation))

//         //if doesnt exist iterate through all existing keys

//         // if key is a transposition -> append permutation to neighbours-> Append key to neighbours
//     }
//     println!("Graph len: {}", graph.nodes.len());
//     println!("Indexing 200000 points: {:?}", start.elapsed());
// }

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
        Index {
            nodes: Vec::new(),
            index_map: hashbrown::HashMap::new(),
        }
    }

    fn add_permutation(&mut self, node: Node) {
        let id = node.id;
        self.nodes.push(node);
        let index = self.nodes.len() - 1;
        self.index_map.insert(id, index);
    }

    fn add_subpermutation(&mut self, subpermutation: u128, data_points:Vec<usize>) {
        let neighbours: Vec<usize> = Vec::new();
        let subpermutation_index = self.nodes.len() - 1;
        for transposition in all_transpositions(subpermutation) {
            match self.index_map.get(transposition) {
                Some(index) => {
                    //if exists add indexes to both
                    self.nodes[*index].neighbours.push(subpermutation_index);
                    neighbours.push(*index);
                }
                None => continue,
            }
        }
        self.add_permutation(Node {
            id: subpermutation,
            data_points: data_points,
            neighbours: neighbours,
        });
    }

    fn add_new_permutation(&mut self, binary: u128, data_point: usize) {
        //for each sibpermutation:

        //for each translation:
        self.add_subpermutation(subpermutation);

        //if it exists
    }

    fn insert(&mut self, point: Vec<T>, data_point: usize) {
        //icalcualte distance to simplexes
        let binary = distance_to_simplex(point, &self.simplexes);

        //check if permutation exists
        match self.index_map.get(binary) {
            Some(index) => {
                //if it does add data point to it
                self.nodes[*index].data_points.push(data_point);
            }
            None => {
                //else add it
                self.add_new_permutation(binary, data_point);
            }
        }
    }
}
