use itertools::Itertools;
use ndarray::Array2;

const INPUT: &str = include_str!("../../inputs/04");

fn input_matrix() -> Array2<char> {
    let mut matrix: Array2<char> = Array2::from_elem((256, 256), '.');

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x + 1;
            let y = y + 1;

            matrix[(x, y)] = c;
        }
    }

    matrix
}

fn occupied_neighbours(matrix: &Array2<char>, (x, y): (usize, usize)) -> usize {
    (-1i32..=1)
        .cartesian_product(-1i32..=1)
        .filter(|pos| *pos != (0, 0))
        .map(|(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
        .filter(|pos| matrix[*pos] == '@')
        .count()
}

fn remove_accessible(matrix: &mut Array2<char>) -> usize {
    let to_remove = (1..256usize)
        .cartesian_product(1..256usize)
        .filter(|pos| matrix[*pos] == '@')
        .filter(|pos| occupied_neighbours(matrix, *pos) < 4)
        .collect_vec();

    for pos in to_remove.iter() {
        matrix[*pos] = '.';
    }

    to_remove.len()
}

fn main() {
    let mut matrix = input_matrix();

    let r: usize = std::iter::repeat_with(|| remove_accessible(&mut matrix))
        .take_while(|removed| *removed > 0)
        .sum();

    println!("{r}");
}
