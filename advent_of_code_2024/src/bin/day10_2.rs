use itertools::Itertools;
use ndarray::{Array2, Axis};

const INPUT: &str = include_str!("../../inputs/10");

fn load_matrix() -> Array2<i8> {
    let height = INPUT.lines().count();
    let width = INPUT.lines().next().unwrap().len();

    let mut matrix: Array2<i8> = Array2::from_elem((width + 2, height + 2), -1);

    for (y, line) in INPUT.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            matrix[(x + 1, y + 1)] = ch.to_digit(10).unwrap() as i8;
        }
    }

    matrix
}

fn go(map: &Array2<i8>, (x, y): (usize, usize)) -> u64 {
    if map[(x, y)] == 9 {
        return 1;
    }

    let next = map[(x, y)] + 1;

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .filter(|(nx, ny)| map[(*nx, *ny)] == next)
        .map(|pos| go(map, pos))
        .sum()
}

fn main() {
    let map = load_matrix();

    let r: u64 = (0..map.len_of(Axis(0)))
        .cartesian_product(0..map.len_of(Axis(1)))
        .filter(|(x, y)| map[(*x, *y)] == 0)
        .map(|pos| go(&map, pos))
        .sum();

    println!("{r}");
}
