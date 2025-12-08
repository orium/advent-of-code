use disjoint::DisjointSet;
use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/08");

#[derive(Debug)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    fn dist_squared(&self, other: &Point) -> u64 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)) as u64
    }
}

struct Edge {
    a: usize,
    b: usize,
}

impl Edge {
    fn weight(&self, vertices: &[Point]) -> u64 {
        vertices[self.a].dist_squared(&vertices[self.b])
    }
}

fn go_kruskal(vertices: &[Point], mut edges: Vec<Edge>, joins: usize) -> DisjointSet {
    let mut disjoint_sets: DisjointSet = DisjointSet::with_len(vertices.len());

    edges.sort_by_key(|edge| edge.weight(vertices));

    for edge in edges.into_iter().take(joins) {
        disjoint_sets.join(edge.a, edge.b);
    }

    disjoint_sets
}

fn main() {
    let points: Vec<Point> = INPUT
        .lines()
        .map(|line| scan_fmt!(line, "{},{},{}", i64, i64, i64).unwrap())
        .map(|(x, y, z)| Point { x, y, z })
        .collect();
    let edges: Vec<Edge> =
        (0..points.len()).combinations(2).map(|v| Edge { a: v[0], b: v[1] }).collect();

    let disjoint_sets = go_kruskal(&points, edges, 1000);

    let r: usize = disjoint_sets.sets().iter().map(Vec::len).sorted().rev().take(3).product();

    println!("{r}");
}
