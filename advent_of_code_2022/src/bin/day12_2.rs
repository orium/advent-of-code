use ndarray::Array2;
use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../inputs/12");

fn bfs(matrix: &mut Array2<i8>, start: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut enqueued: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((0, start));
    enqueued.insert(start);

    while let Some((count, (x, y))) = queue.pop_front() {
        if matrix[(x, y)] == 0 {
            return Some(count);
        }

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ax, ay) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            if matrix[(x, y)] <= matrix[(ax, ay)] + 1 && !enqueued.contains(&(ax, ay)) {
                queue.push_back((count + 1, (ax, ay)));
                enqueued.insert((ax, ay));
            }
        }
    }

    None
}

fn main() {
    let mut matrix: Array2<i8> = Array2::from_elem((256, 256), -100);
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    for (y, row) in INPUT.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            // Make sure we have sentinel values around the entire matrix.
            let (x, y) = (x + 1, y + 1);

            match c {
                'S' => start = Some((x, y)),
                'E' => end = Some((x, y)),
                _ => matrix[(x, y)] = ((c as u32) - ('a' as u32)) as i8,
            }
        }
    }

    let start: (usize, usize) = start.unwrap();
    let end: (usize, usize) = end.unwrap();

    matrix[start] = 0;
    matrix[end] = (('z' as u32) - ('a' as u32)) as i8;

    println!("{:?}", bfs(&mut matrix, end));
}
