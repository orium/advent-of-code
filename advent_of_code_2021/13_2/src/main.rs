use std::fmt::{Debug, Formatter, Write};
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Grid {
    cells: [[bool; 1400]; 1400],
    transposed: bool,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: [[false; 1400]; 1400],
            transposed: false,
        }
    }

    fn coords(&self, x: usize, y: usize) -> (usize, usize) {
        match self.transposed {
            true => (y, x),
            false => (x, y),
        }
    }

    fn set(&mut self, x: usize, y: usize, p: bool) {
        let (x, y) = self.coords(x, y);
        self.cells[x][y] = p;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let (x, y) = self.coords(x, y);
        self.cells[x][y]
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..10 {
            for x in 0..60 {
                match self.get(x, y) {
                    true => f.write_char('█')?,
                    false => f.write_char(' ')?,
                };
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn fold_y(grid: &mut Grid, line: usize) {
    for y in line..1400 {
        for x in 0..1400 {
            if grid.get(x, y) {
                grid.set(x, y, false);
                grid.set(x, line - (y - line), true);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut lines = reader.lines();
    let mut grid = Grid::new();

    while let Some(line) = lines.next() {
        let line: String = line?.parse().unwrap();

        if line.trim().is_empty() {
            break;
        }

        let (x, y) = line.split_at(line.find(",").unwrap());
        let (x, y) = (usize::from_str(x).unwrap(), usize::from_str(&y[1..]).unwrap());

       grid.set(x, y, true);
    }

    while let Some(line) = lines.next() {
        let line: String = line?.parse().unwrap();

        if !line.starts_with("fold along") {
            continue;
        }

        let (_, fold_str) = line.split_at("fold along ".len());
        let (c, v) = fold_str.split_at("x=".len());
        let (c, v) = (c.chars().next().unwrap(), usize::from_str(v).unwrap());

        match c {
            'x' => {
                grid.transpose();
                fold_y(&mut grid, v);
                grid.transpose();
            },
            'y' => fold_y(&mut grid, v),
            _ => unreachable!(),
        }

        println!("{:?}", grid);
    }

    Ok(())
}
