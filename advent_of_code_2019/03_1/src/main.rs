use ndarray::Array2;
use std::borrow::Borrow;
use std::io;
use std::io::{BufRead, BufReader};

const LIMIT: usize = 100_000;
const CENTER: (usize, usize) = (LIMIT / 2, LIMIT / 2);

fn distance_from_center(position: (usize, usize)) -> usize {
    ((position.0 as isize - CENTER.0 as isize).abs()
        + (position.1 as isize - CENTER.1 as isize).abs()) as usize
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut matrix: Array2<u8> = Array2::from_elem((LIMIT, LIMIT), 0);
    let mut min_distance = std::usize::MAX;

    for (line, wire_id) in reader.lines().zip(1..) {
        let moves: Vec<(String, u32)> = line
            .unwrap()
            .split(',')
            .map(|v| (v[0..1].to_owned(), v[1..].parse::<u32>().unwrap()))
            .collect();
        let mut position: (usize, usize) = CENTER;

        for (direction, distance) in moves {
            let (dh, dv): (isize, isize) = match direction.borrow() {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!(),
            };

            for _ in 0..distance {
                position =
                    ((position.0 as isize + dh) as usize, (position.1 as isize + dv) as usize);
                if matrix[position] > 0 && matrix[position] != wire_id {
                    min_distance = distance_from_center(position).min(min_distance);
                }
                matrix[position] = wire_id;
            }
        }
    }

    if false {
        for y in (0..LIMIT).rev() {
            for x in 0..LIMIT {
                if matrix[(x, y)] > 0 {
                    print!("{}", matrix[(x, y)]);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    println!("{}", min_distance);

    Ok(())
}
