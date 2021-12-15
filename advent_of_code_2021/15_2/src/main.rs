use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io;
use std::io::{BufRead, BufReader};

struct Grid {
    cells: Vec<Vec<u8>>
}

impl Grid {
    fn rows(&self) -> usize {
        5 * self.cells.len()
    }

    fn cols(&self) -> usize {
        5 * self.cells[0].len()
    }

    fn get(&self, i: usize, j: usize) -> u8 {
        let tile_rows = self.rows() / 5;
        let tile_cols = self.cols() / 5;

        let v: u8 = self.cells[i % tile_rows][j % tile_cols];
        let v = v + (i / tile_rows) as u8 + (j / tile_cols) as u8;

        v % 10 + v / 10
    }

    fn valid_pos(i: isize, j: isize, rows: usize, cols: usize) -> bool {
        i >= 0 && i < rows as isize && j >= 0 && j < cols as isize
    }

    fn adjacents(&self, (i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows();
        let cols = self.cols();

        [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter()
            .map(move |(di, dj)| (i as isize + di, j as isize + dj))
            .filter(move |(ai, aj)| Self::valid_pos(*ai, *aj, rows, cols))
            .map(|(ai, aj)| (ai as usize, aj as usize))
    }
}

struct State {
    coords: (usize, usize),
    dist: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn dijkstra(grid: &Grid, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut queue: BinaryHeap<State> = BinaryHeap::with_capacity(grid.rows() * grid.cols());
    let mut enqueued: HashSet<(usize, usize)> = HashSet::new();

    queue.push(State { coords: start, dist: 0 });
    enqueued.insert(start);

    while let Some(state) = queue.pop() {
        if state.coords == end {
            return Some(state.dist);
        }

        for (i, j) in grid.adjacents(state.coords) {
            if !enqueued.contains(&(i, j)) {
                queue.push(State { coords: (i, j), dist: state.dist + grid.get(i, j) as usize});
                enqueued.insert((i, j));
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut grid = Grid { cells: Vec::with_capacity(128) };

    for line in reader.lines() {
        let line = line.unwrap();

        grid.cells.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }

    let dist = dijkstra(&grid, (0, 0), (grid.rows() - 1, grid.cols() - 1)).unwrap();

    println!("{}", dist);

    Ok(())
}
