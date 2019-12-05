use ndarray::Array2;
use std::borrow::Borrow;
use std::io;
use std::io::{BufRead, BufReader};

const LIMIT: usize = 25_000;
const CENTER: (usize, usize) = (LIMIT / 2, LIMIT / 2);

#[derive(Clone, Copy)]
struct ReachedBy {
    distance: [u32; 2], // Distance by wire_id
}

impl ReachedBy {
    fn combined_distance(&self) -> u32 {
        self.distance[0].saturating_add(self.distance[1])
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut matrix: Array2<ReachedBy> =
        Array2::from_elem((LIMIT, LIMIT), ReachedBy { distance: [std::u32::MAX; 2] });
    let mut best_combined_distance = std::u32::MAX;

    for (line, wire_id) in reader.lines().zip(0..) {
        let moves: Vec<(String, u32)> = line
            .unwrap()
            .split(',')
            .map(|v| (v[0..1].to_owned(), v[1..].parse::<u32>().unwrap()))
            .collect();
        let mut position: (usize, usize) = CENTER;
        let mut wire_length = 0;

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
                wire_length += 1;

                matrix[position].distance[wire_id] =
                    wire_length.min(matrix[position].distance[wire_id]);

                best_combined_distance =
                    (matrix[position].combined_distance()).min(best_combined_distance);
            }
        }
    }

    println!("{}", best_combined_distance);

    Ok(())
}
