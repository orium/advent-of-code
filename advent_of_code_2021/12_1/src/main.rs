use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Graph {
    adj: HashMap<String, Vec<String>>
}

impl Graph {
    fn new() -> Graph {
        Graph { adj: HashMap::new() }
    }

    fn add_edge(&mut self, s: String, d: String) {
        self.adj.entry(s).or_default().push(d);
    }
}

fn is_small(node: &str) -> bool {
    node.chars().next().unwrap().is_lowercase()
}

fn explore(graph: &Graph, node: &str, small_visited: &mut HashSet<String>) -> usize {
    if node == "end" {
        return 1;
    }

    if is_small(node) {
        if small_visited.contains(node) {
            return 0;
        }
        small_visited.insert(node.to_owned());
    }

    let mut paths = 0;

    let succ = if let Some(s) = graph.adj.get(node) { s } else { return 0 };

    for v in succ {
        paths += explore(graph, v.as_str(), small_visited);
    }

    if is_small(node) {
        small_visited.remove(node);
    }

    paths
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut graph: Graph = Graph::new();

    for line in reader.lines() {
        let line: String = line?.parse().unwrap();
        let (s, wd) = line.split_at(line.find("-").unwrap());

        graph.add_edge(s.to_owned(), wd[1..].to_owned());
        graph.add_edge(wd[1..].to_owned(), s.to_owned(),);
    }

    let paths = explore(&graph, "start", &mut HashSet::new());

    println!("{}", paths);

    Ok(())
}
