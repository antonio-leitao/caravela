
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

fn path_from_origin(mut x: u128) -> impl Iterator<Item = u128> {
    let mut flips = Vec::new();
    while x > 0 {
        flips.push(x);
        x = flip_last_one_to_zero(x);
    }
    flips.into_iter().rev()
}

fn flip_last_one_to_zero(n: u128) -> u128 {
    n & (n - 1)
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

    fn add_subpermutation(&mut self, subpermutation: u128, data_points: Vec<usize>) {
        let mut neighbours: Vec<usize> = Vec::new();
        let subpermutation_index = self.nodes.len() - 1;
        for transposition in all_transpositions(subpermutation) {
            match self.index_map.get(&transposition) {
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
            data_points:data_points,
            neighbours: neighbours,
        });
    }

    fn add_new_permutation(&mut self, permutation: u128, data_point: usize) {
        //add all subpermutations from origin to permutation
        for subpermutation in path_from_origin(permutation){
            let mut data_points=Vec::new();
            if subpermutation==permutation{
                data_points.push(data_point);
            }
            self.add_subpermutation(subpermutation, data_points);
        }
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

fn main(){
    let mut index:Index = Index::new();
    const N:usize = 2000;
    const K:usize =16;
    let permutations = utils::random_permutations(N,K);
    for (data_point,permutation) in permutations.iter().enumerate(){
       index.insert(permutation_to_u128(&permutation),data_point);
    }
}


