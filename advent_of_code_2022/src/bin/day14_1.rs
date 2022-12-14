use itertools::Itertools;
use ndarray::Array2;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/14");

fn add_line(matrix: &mut Array2<char>, (sx, sy): (usize, usize), (ex, ey): (usize, usize)) {
    for (x, y) in (sx.min(ex)..=sx.max(ex)).cartesian_product(sy.min(ey)..=sy.max(ey)) {
        matrix[(x, y)] = '█';
    }
}

fn print(matrix: &Array2<char>) {
    for y in 0..11 {
        for x in 493..504 {
            print!("{}", matrix[(x, y)])
        }
        println!();
    }
}

fn fall_vertically(
    matrix: &Array2<char>,
    (sx, sy): (usize, usize),
    bottom: usize,
) -> Option<(usize, usize)> {
    (sy..=bottom).map(|y| (sx, y)).take_while(|(x, y)| matrix[(*x, *y)] == ' ').last()
}

fn topple(matrix: &Array2<char>, (x, y): (usize, usize)) -> (usize, usize) {
    [(x - 1, y + 1), (x + 1, y + 1)].into_iter().find(|p| matrix[*p] == ' ').unwrap_or((x, y))
}

fn fall(matrix: &Array2<char>, (sx, sy): (usize, usize), bottom: usize) -> Option<(usize, usize)> {
    let (mut x, mut y) = (sx, sy);

    loop {
        let new_point = fall_vertically(matrix, (x, y), bottom).map(|p| topple(matrix, p));

        match new_point {
            None => return None,
            Some(p) if p == (x, y) => return Some(p),
            Some(p) => (x, y) = p,
        }
    }
}

fn main() {
    let mut matrix: Array2<char> = Array2::from_elem((1024, 1024), ' ');
    let mut bottom = 0;

    for line in INPUT.lines() {
        let pairs = line
            .split(" -> ")
            .map(|s| scan_fmt!(s, "{},{}", usize, usize).unwrap())
            .tuple_windows();

        for ((sx, sy), (ex, ey)) in pairs {
            add_line(&mut matrix, (sx, sy), (ex, ey));
            bottom = bottom.max(sy).max(ey);
        }
    }

    bottom += 1;

    for i in 0.. {
        let Some((fx, fy)) = fall(&matrix, (500, 0), bottom) else {
            println!("{}", i);
            return;
        };

        matrix[(fx, fy)] = '●';

        print(&matrix);
    }
}
