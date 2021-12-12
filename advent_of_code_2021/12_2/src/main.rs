use std::collections::HashMap;
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

fn explore(
    graph: &Graph,
    node: &str,
    small_visited: &mut HashMap<String, usize>,
    small_exhausted: bool,
) -> usize {
    let mut is_repeated = false;

    if node == "end" {
        return 1;
    }

    if is_small(node) {
        if small_visited.contains_key(node) {
            if small_exhausted {
                return 0;
            } else {
                is_repeated = true;
            }
        }
        *small_visited.entry(node.to_owned()).or_default() += 1;
    }

    let mut paths = 0;

    let succ = if let Some(s) = graph.adj.get(node) { s } else { return 0 };

    for v in succ {
        if v == "start" {
            continue;
        }

        paths += explore(graph, v.as_str(), small_visited, small_exhausted || is_repeated);
    }

    if is_small(node) {
        let c = small_visited.entry(node.to_owned()).or_default();
        *c -= 1;
        if *c == 0 {
            small_visited.remove(node);
        }
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

    let paths = explore(&graph, "start", &mut HashMap::new(), false);

    println!("{}", paths);

    Ok(())
}
