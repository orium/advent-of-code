use itertools::Itertools;
use ndarray::{Array2, Axis};

const INPUT: &str = include_str!("../../inputs/04");

const WORD: &str = "MAS";

fn load_matrix() -> Array2<char> {
    let mut matrix: Array2<char> = Array2::from_elem((256, 256), ' ');

    for (x, line) in INPUT.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            matrix[(x + 1, y + 1)] = ch;
        }
    }

    matrix
}

fn is_word(iter: impl Iterator<Item = char>) -> bool {
    iter.take(WORD.len()).eq(WORD.chars())
}

fn find_in_direction(
    matrix: &Array2<char>,
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> bool {
    let x_seq = std::iter::successors(Some(x), |&x| Some(x.saturating_add_signed(dx)));
    let y_seq = std::iter::successors(Some(y), |&y| Some(y.saturating_add_signed(dy)));

    let chars_seq = x_seq.zip(y_seq).map(|coords| matrix[coords]);

    is_word(chars_seq)
}

fn find(matrix: &Array2<char>, (x, y): (usize, usize)) -> usize {
    if matrix[(x, y)] == ' ' {
        return 0;
    }

    ((find_in_direction(matrix, (x, y), (1, 1))
        || find_in_direction(matrix, (x + WORD.len() - 1, y + WORD.len() - 1), (-1, -1)))
        && (find_in_direction(matrix, (x, y + WORD.len() - 1), (1, -1))
            || find_in_direction(matrix, (x + WORD.len() - 1, y), (-1, 1)))) as usize
}

fn main() {
    let matrix = load_matrix();

    let r: usize = (0..matrix.len_of(Axis(0)))
        .cartesian_product(0..matrix.len_of(Axis(1)))
        .map(|coords| find(&matrix, coords))
        .sum();

    println!("{r}");
}
