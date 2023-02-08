use std::collections::VecDeque;
use std::time::Instant;

fn closest_nodes(root: usize, n: usize, adjacency_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; adjacency_list.len()];
    let mut result = vec![];

    queue.push_back(root);
    visited[root] = true;

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if result.len() == n {
            break;
        }

        for &neighbor in &adjacency_list[node] {
            if !visited[neighbor] {
                queue.push_back(neighbor);
                visited[neighbor] = true;
            }
        }
    }

    result
}

fn main() {
    let n = 100000;
    let m = 200000;
    let n_tests = 10;
    let root = 0;
    let n_nodes = 10;

    let mut adjacency_list = vec![Vec::new(); n];
    for i in 0..m {
        let a = (i * i + i) % n;
        let b = (i * i * i + i * i) % n;
        adjacency_list[a].push(b);
    }

    let start = Instant::now();
    for _ in 0..n_tests {
        closest_nodes(root, n_nodes, &adjacency_list);
    }
    let duration = start.elapsed();

    println!("Elapsed time: {:?}", duration);
}
