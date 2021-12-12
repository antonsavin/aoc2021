use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

type Graph = HashMap<String, HashSet<String>>;

fn is_small(node: &str) -> bool {
    return node.chars().next().unwrap().is_lowercase();
}

fn read_graph() -> Graph {
    let reader = BufReader::new(io::stdin());
    let mut graph: Graph = Graph::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.trim().split('-').collect();
        let name1 = fields.get(0).unwrap();
        let name2 = fields.get(1).unwrap();
        graph
            .entry(String::from(*name1))
            .or_insert(HashSet::new())
            .insert(String::from(*name2));
        graph
            .entry(String::from(*name2))
            .or_insert(HashSet::new())
            .insert(String::from(*name1));
    }

    graph
}

fn dfs_1(graph: &Graph, node: &str, visited_small: &HashSet<&str>) -> u64 {
    let mut cnt: u64 = 0;
    for next in graph.get(node).unwrap() {
        if visited_small.contains(next as &str) {
            continue;
        } else if next == "end" {
            cnt += 1;
        } else if is_small(next) {
            if !visited_small.contains(next as &str) {
                let mut new_visited_small = visited_small.clone();
                new_visited_small.insert(next);
                cnt += dfs_1(graph, next, &new_visited_small);
            }
        } else {
            cnt += dfs_1(graph, next, visited_small);
        }
    }

    cnt
}

fn solve_part_1() {
    let graph = read_graph();

    let mut visited_small: HashSet<&str> = HashSet::new();
    visited_small.insert("start");
    let num_paths = dfs_1(&graph, "start", &visited_small);
    println!("Num paths: {}", num_paths);
}

fn dfs_2(
    graph: &Graph,
    node: &str,
    visited_small: &HashSet<&str>,
    visited_some_small_twice: bool,
) -> u64 {
    let mut cnt: u64 = 0;
    for next in graph.get(node).unwrap() {
        if next == "start" {
            continue;
        } else if next == "end" {
            cnt += 1;
        } else if visited_small.contains(next as &str) {
            if !visited_some_small_twice {
                cnt += dfs_2(graph, next, visited_small, true);
            }
        } else if is_small(next) {
            let mut new_visited_small = visited_small.clone();
            new_visited_small.insert(next);
            cnt += dfs_2(graph, next, &new_visited_small, visited_some_small_twice);
        } else {
            cnt += dfs_2(graph, next, visited_small, visited_some_small_twice);
        }
    }

    cnt
}

fn solve_part_2() {
    let graph = read_graph();

    let visited_small: HashSet<&str> = HashSet::new();
    let num_paths = dfs_2(&graph, "start", &visited_small, false);
    println!("Num paths: {}", num_paths);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_part_1 = args[1] == "1";
    if is_part_1 {
        solve_part_1();
    } else {
        solve_part_2();
    }
}
