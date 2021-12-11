use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::io;
use std::io::{BufRead, BufReader};

struct Grid {
    cells: Vec<Vec<u8>>
}

impl Grid {
    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }

    fn valid_pos(&self, i: isize, j: isize) -> bool {
        i >= 0 && i < self.rows() as isize && j >= 0 && j < self.cols() as isize
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for row in &self.cells {
            if !first {
                f.write_char('\n')?;
            }
            for &c in row {
                f.write_char(char::from_digit(c as u32, 10).unwrap())?;
            }
            first = false;
        }

        Ok(())
    }
}

fn step(grid: &mut Grid) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(grid.rows() * grid.cols());
    let mut enqueued: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            grid.cells[i][j] += 1;
            if grid.cells[i][j] > 9 && !enqueued.contains(&(i, j)) {
                queue.push_back((i, j));
                enqueued.insert((i, j));
            }
        }
    }

    while let Some((i, j)) = queue.pop_front() {

        for di in -1..=1 {
            for dj in -1..=1 {
                if grid.valid_pos(i as isize + di, j as isize + dj)  {
                    let (ai, aj) = ((i as isize + di) as usize, (j as isize + dj) as usize);

                    grid.cells[ai][aj] = grid.cells[ai][aj].saturating_add(1);

                    if grid.cells[ai][aj] > 9 && !enqueued.contains(&(ai, aj)) {
                        queue.push_back((ai, aj));
                        enqueued.insert((ai, aj));
                    }
                }
            }
        }
    }

    for (i, j) in &enqueued {
        grid.cells[*i][*j] = 0;
    }

    enqueued.len()
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut grid: Grid = Grid { cells: Vec::new() };

    for line in reader.lines() {
        let line: String = line?.parse().unwrap();
        let row: Vec<u8> = line.chars().map(|ch| char::to_digit(ch, 10).unwrap() as u8).collect();

        grid.cells.push(row);
    }

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    println!("{}", flashes);

    Ok(())
}
