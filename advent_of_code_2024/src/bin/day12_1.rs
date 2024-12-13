use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/12");

fn load_matrix() -> Array2<char> {
    let height = INPUT.lines().count();
    let width = INPUT.lines().next().unwrap().len();

    let mut matrix: Array2<char> = Array2::from_elem((width + 2, height + 2), ' ');

    for (y, line) in INPUT.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            matrix[(x + 1, y + 1)] = ch;
        }
    }

    matrix
}

fn fill(
    map: &Array2<char>,
    visited: &mut HashSet<(usize, usize)>,
    (x, y): (usize, usize),
) -> (usize, usize) {
    let region: char = map[(x, y)];

    visited.insert((x, y));

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .map(|neighbour| match (visited.contains(&neighbour), map[neighbour] == region) {
            (true, true) => (0, 0),
            (true, false) => (1, 0),
            (false, true) => fill(map, visited, neighbour),
            (false, false) => (1, 0),
        })
        .reduce(|(p0, a0), (p1, a1)| (p0 + p1, a0 + a1))
        .map(|(p, a)| (p, a + 1))
        .unwrap()
}

fn main() {
    let map = load_matrix();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let r: usize = (0..map.len_of(Axis(0)))
        .cartesian_product(0..map.len_of(Axis(1)))
        .filter(|&pos| map[pos] != ' ')
        .map(|pos| match visited.contains(&pos) {
            true => (0, 0),
            false => fill(&map, &mut visited, pos),
        })
        .map(|(p, a)| p * a)
        .sum();

    println!("{r}");
}
