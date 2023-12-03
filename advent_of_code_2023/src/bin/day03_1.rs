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

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn adjacents((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1isize)
        .cartesian_product(-1..=1isize)
        .filter(|(dx, dy)| !(*dx == 0 && *dy == 0))
        .map(move |(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
}

fn has_symbol_around(matrix: &Array2<char>, (x, y): (usize, usize)) -> bool {
    adjacents((x, y)).any(|pos| is_symbol(matrix[pos]))
}

fn main() {
    let matrix = load_matrix();

    let mut total: u64 = 0;

    let mut current_num: Option<u64> = None;
    let mut has_symbol: bool = false;

    for (y, x) in (1..250).cartesian_product(1..250) {
        match (current_num, matrix[(x, y)].to_digit(10)) {
            (None, None) => (),
            (None, Some(d)) => {
                current_num = Some(d as u64);
                has_symbol = has_symbol_around(&matrix, (x, y));
            }
            (Some(n), None) => {
                if has_symbol {
                    total += n;
                }
                current_num = None;
                has_symbol = false;
            }
            (Some(n), Some(d)) => {
                current_num = Some(n * 10 + d as u64);
                has_symbol = has_symbol || has_symbol_around(&matrix, (x, y));
            }
        }
    }

    println!("{total}");
}
