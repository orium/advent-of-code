use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader, Read};

struct Graph<T> {
    adj_list: HashMap<T, Vec<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash,
{
    fn new() -> Graph<T> {
        Graph { adj_list: HashMap::new() }
    }

    fn add_edge(&mut self, origin: T, destination: T) {
        self.adj_list.entry(origin).or_default().push(destination);
    }

    fn edges<B: ?Sized>(&self, node: &B) -> Option<impl Iterator<Item = &T>>
    where
        T: Borrow<B>,
        B: Eq + Hash,
    {
        self.adj_list.get(node.borrow()).map(|adjs| adjs.iter())
    }

    fn nodes(&self) -> impl Iterator<Item = &T> {
        self.adj_list.keys()
    }
}

fn read_graph<R: Read>(reader: R) -> io::Result<Graph<String>> {
    let reader = BufReader::new(reader);
    let mut graph: Graph<String> = Graph::new();

    for line in reader.lines() {
        let line: String = line?;

        if line == "END" {
            break;
        }

        let mut it = line.split(')');
        let center = it.next().unwrap().trim().to_owned();
        let orbiter = it.next().unwrap().trim().to_owned();

        graph.add_edge(orbiter, center);
    }

    Ok(graph)
}

fn count_reachable_by<'a>(graph: &'a Graph<String>, node: &'a str) -> u32 {
    let mut count = 0;

    if let Some(it) = graph.edges(node) {
        for succ in it {
            count += 1 + count_reachable_by(graph, succ);
        }
    }

    count
}

fn main() -> io::Result<()> {
    let graph = read_graph(io::stdin())?;
    let mut count = 0;

    for node in graph.nodes() {
        count += count_reachable_by(&graph, node);
    }

    println!("{}", count);

    Ok(())
}
