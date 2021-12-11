use std::fmt::{Display, Formatter, Write};
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

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

    fn valid_pos(i: isize, j: isize, rows: usize, cols: usize) -> bool {
        i >= 0 && i < rows as isize && j >= 0 && j < cols as isize
    }

    fn adjacent(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows();
        let cols = self.cols();

        (-1isize..=1).cartesian_product(-1..=1)
            .filter(|&p| p != (0, 0))
            .map(move |(di, dj)| (i as isize + di, j as isize + dj))
            .filter(move |(ai, aj)| Self::valid_pos(*ai, *aj, rows, cols))
            .map(|(ai, aj)| (ai as usize, aj as usize))
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
    let mut queue: Vec<(usize, usize)> = Vec::with_capacity(grid.rows() * grid.cols());
    let mut queue_next = 0;

    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            grid.cells[i][j] += 1;
            if grid.cells[i][j] == 10 {
                queue.push((i, j));
            }
        }
    }

    while queue_next < queue.len() {
        let (i, j) = queue[queue_next];
        queue_next += 1;

        for (ai, aj) in grid.adjacent(i, j) {
            grid.cells[ai][aj] = grid.cells[ai][aj].saturating_add(1);

            if grid.cells[ai][aj] == 10 {
                queue.push((ai, aj));
            }
        }
    }

    for (i, j) in &queue {
        grid.cells[*i][*j] = 0;
    }

    queue.len()
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut grid: Grid = Grid { cells: Vec::new() };

    for line in reader.lines() {
        let line: String = line?.parse().unwrap();
        let row: Vec<u8> = line.chars().map(|ch| char::to_digit(ch, 10).unwrap() as u8).collect();

        grid.cells.push(row);
    }

    for i in 1.. {
        if step(&mut grid) == grid.rows() * grid.cols() {
            println!("{}", i);
            break;
        }
    }

    Ok(())
}
