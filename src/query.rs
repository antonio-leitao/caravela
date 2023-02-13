//get distance to simplexes
//create binary vecotr
//get gradient vector

// POSITION
//check for direct match
//if not start removing last bits untill match.

//SEARCH
//Calculate distance of curretn node and neighbour nodes
//add all calculated nodes to heap [be careful with repetition here]
//add current node to evaluated list
//Check if there is better node in heap

// YES: move to better node
// NO: add current node to results list, remove current node from heap

//continue untill results list is k in size.
//return results list


use std::collections::BinaryHeap;
use std::time::Instant;

// Define the Node struct to represent nodes in the graph
#[derive(Debug, PartialEq)]
struct Node {
    value: i32,
    distance: i32,
}

// Implement the Ord trait for Node to make it usable in a BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        // Implement a max-heap, so the node with the largest distance
        // will be the top element of the heap
        other.distance.cmp(&self.distance)
    }
}

// Implement the PartialOrd trait for Node
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let num_nodes = 2000;
    let k = 20;

    // Generate a sample graph with random values for each node
    let mut graph: Vec<Node> = Vec::with_capacity(num_nodes);
    for i in 0..num_nodes {
        graph.push(Node {
            value: i as i32,
            distance: (i as f32 * 100.0).round() as i32,
        });
    }

    // Define the target point T
    let t = Node {
        value: -1,
        distance: 1500,
    };

    let mut total_time = 0;
    let num_trials = 100;
    for _ in 0..num_trials {
        let start = Instant::now();

        // Initialize the priority queue with the first node
        let mut heap = BinaryHeap::new();
        heap.push(graph[0]);

        // Keep track of the nodes we've already visited
        let mut visited = vec![false; num_nodes];
        visited[0] = true;

        // Keep track of the k closest nodes
        let mut closest_nodes = Vec::with_capacity(k);

        // Repeat until we've found k closest nodes or the priority queue is empty
        while closest_nodes.len() < k && !heap.is_empty() {
            // Get the node with the highest priority (i.e., the largest distance)
            let current = heap.pop().unwrap();

            // Add the current node to the list of closest nodes
            closest_nodes.push(current);

            // Check all neighbors of the current node
            for i in 0..num_nodes {
                let node = &graph[i];
                if visited[i] || node.distance >= current.distance {
                    continue;
                }

                // Calculate the distance from the node to the target point T
                let distance = (node.distance - t.distance).abs();

                // Add the node to the priority queue and mark it as visited
                heap.push(Node { value: node.value, distance });
                visited[i] = true;
            }
        }

        // Record the time
        let elapsed = start.elapsed();
        total_time += elapsed.as_micros();
    }

    // Calculate the average time
    let average_time = total_time as f64 / num_trials as f64;
    println!("Average time to find {} closest nodes: {} microseconds", k, average_time);
}


