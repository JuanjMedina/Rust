use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read n (number of jobs) and m (number of relations)
    let first_line = lines.next().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let m: usize = first_iter.next().unwrap().parse().unwrap();

    // Build the graph and indegree vector
    let mut graph = vec![Vec::new(); n + 1];
    let mut indegree = vec![0; n + 1];
    
    for _ in 0..m {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        graph[u].push(v);
        indegree[v] += 1;
    }
    
    // Use a min-heap to get lexicographically smallest order
    let mut heap = BinaryHeap::new();
    for job in 1..=n {
        if indegree[job] == 0 {
            heap.push(Reverse(job));
        }
    }
    
    let mut order = Vec::new();
    
    while let Some(Reverse(u)) = heap.pop() {
        order.push(u);
        // For each dependent job, reduce the indegree
        for &v in &graph[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                heap.push(Reverse(v));
            }
        }
    }
    
    // Check if we have processed all jobs
    if order.len() == n {
        // Print the ordering space-separated
        let result: Vec<String> = order.iter().map(|job| job.to_string()).collect();
        println!("{}", result.join(" "));
    } else {
        println!("Sandro fails.");
    }
}
