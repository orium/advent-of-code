use ndarray::Array2;

const INPUT: &str = include_str!("../../inputs/08");

fn viewing_distance(
    matrix: &Array2<i8>,
    start: (usize, usize),
    (step_x, step_y): (isize, isize),
) -> usize {
    let (mut x, mut y): (isize, isize) = (start.0 as isize, start.1 as isize);
    let mut count = 0;
    let start_height = matrix[(x as usize, y as usize)];

    x += step_x;
    y += step_y;

    while x >= 0 && y >= 0 && matrix[(x as usize, y as usize)] >= 0 {
        let height = matrix[(x as usize, y as usize)];

        count += 1;

        if height >= start_height {
            break;
        }

        x += step_x;
        y += step_y;
    }

    count
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

    let mut best: usize = 0;

    for y in 0..height {
        for x in 0..width {
            let score = viewing_distance(&matrix, (x, y), (0, 1))
                * viewing_distance(&matrix, (x, y), (0, -1))
                * viewing_distance(&matrix, (x, y), (1, 0))
                * viewing_distance(&matrix, (x, y), (-1, 0));

            best = best.max(score);
        }
    }

    println!("{}", best);
}
