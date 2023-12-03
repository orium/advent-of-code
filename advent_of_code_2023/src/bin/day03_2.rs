use itertools::Itertools;
use ndarray::Array2;

const INPUT: &str = include_str!("../../inputs/03");

fn load_matrix() -> Array2<char> {
    let mut matrix: Array2<char> = Array2::from_elem((256, 256), '.');

    for (y, line) in INPUT.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            matrix[(x + 1, y + 1)] = ch;
        }
    }

    matrix
}

fn adjacents((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1isize)
        .cartesian_product(-1..=1isize)
        .filter(|(dx, dy)| !(*dx == 0 && *dy == 0))
        .map(move |(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
}

fn dig_number_start(matrix: &Array2<char>, (x, y): (usize, usize)) -> Option<(usize, usize)> {
    (0..=x).rev().take_while(|xx| matrix[(*xx, y)].is_ascii_digit()).last().map(|xx| (xx, y))
}

fn number(matrix: &Array2<char>, (x, y): (usize, usize)) -> u64 {
    (0..).map_while(|dx| matrix[(x + dx, y)].to_digit(10)).fold(0, |n, d| n * 10 + d as u64)
}

fn main() {
    let matrix = load_matrix();

    let v: u64 = (1..250)
        .cartesian_product(1..250)
        .filter(|pos| matrix[*pos] == '*')
        .filter_map(|pos| {
            adjacents(pos)
                .filter_map(|pos| dig_number_start(&matrix, pos))
                .unique()
                .collect_tuple()
                .map(|(pos_0, pos_1)| number(&matrix, pos_0) * number(&matrix, pos_1))
        })
        .sum();

    println!("{v}");
}
