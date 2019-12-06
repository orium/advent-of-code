use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub struct Graph<T> {
    adj_list: HashMap<T, Vec<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Graph<T> {
        Graph { adj_list: HashMap::new() }
    }

    pub fn add_edge(&mut self, origin: T, destination: T) {
        self.adj_list.entry(origin).or_default().push(destination);
    }

    pub fn successors<B: ?Sized>(&self, node: &B) -> Option<impl Iterator<Item = &T>>
    where
        T: Borrow<B>,
        B: Eq + Hash,
    {
        self.adj_list.get(node.borrow()).map(|adjs| adjs.iter())
    }

    pub fn nodes(&self) -> impl Iterator<Item = &T> {
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

        graph.add_edge(orbiter.clone(), center.clone());
        graph.add_edge(center, orbiter);
    }

    Ok(graph)
}

fn distance_bfs(graph: &Graph<String>, start: &str, dest: &str) -> Option<u32> {
    let mut queue: VecDeque<(u32, &str)> = VecDeque::new();
    let mut enqueue: HashSet<String> = HashSet::new();

    queue.push_back((0, start));
    enqueue.insert(start.to_owned());

    while let Some((distance, node)) = queue.pop_front() {
        if node == dest {
            return Some(distance);
        }
        if let Some(it) = graph.successors(node) {
            for succ in it {
                if !enqueue.contains(succ) {
                    queue.push_back((distance + 1, succ));
                    enqueue.insert(succ.clone());
                }
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let graph = read_graph(io::stdin())?;
    let distance = distance_bfs(&graph, &"YOU", &"SAN").unwrap() - 2;

    println!("{}", distance);

    Ok(())
}
