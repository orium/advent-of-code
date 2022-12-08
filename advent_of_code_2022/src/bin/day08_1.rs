use ndarray::Array2;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/08");

fn count_visible(
    matrix: &Array2<i8>,
    start: (usize, usize),
    (step_x, step_y): (isize, isize),
    visible: &mut HashSet<(usize, usize)>,
) {
    let (mut x, mut y): (isize, isize) = (start.0 as isize, start.1 as isize);
    let mut max_height: i8 = -1;

    while x >= 0 && y >= 0 && matrix[(x as usize, y as usize)] >= 0 {
        let height = matrix[(x as usize, y as usize)];

        if height > max_height {
            max_height = height;
            visible.insert((x as usize, y as usize));
        }

        x += step_x;
        y += step_y;
    }
}

fn main() {
    let mut matrix: Array2<i8> = Array2::from_elem((128, 128), -1);
    let mut height = 0;
    let mut width = 0;

    for (y, row) in INPUT.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if let Some(h) = ch.to_digit(10) {
                matrix[(x, y)] = h as i8;
                width = width.max(x + 1);
            }
        }
        height = height.max(y + 1);
    }

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..height {
        count_visible(&matrix, (0, y), (1, 0), &mut visible);
        count_visible(&matrix, (width - 1, y), (-1, 0), &mut visible);
    }

    for x in 0..width {
        count_visible(&matrix, (x, 0), (0, 1), &mut visible);
        count_visible(&matrix, (x, height - 1), (0, -1), &mut visible);
    }

    println!("{}", visible.len());
}
