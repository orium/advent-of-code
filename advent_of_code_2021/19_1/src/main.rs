use std::fmt::{Display, Formatter, Write};
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Point(i64, i64, i64);

impl Point {
    fn set(&self, index: usize, value: i64) -> Point {
        match index {
            0 => Point(value, self.1, self.2),
            1 => Point(self.0, value, self.2),
            2 => Point(self.0, self.1, value),
            _ => unreachable!(),
        }
    }

    fn update(&self, index: usize, f: impl Fn(i64) -> i64) -> Point {
        self.set(index, f(self.get(index)))
    }

    fn get(&self, index: usize) -> i64 {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => unreachable!(),
        }
    }

    fn swap(&self, i: usize, j: usize) -> Point {
        let vi = self.get(i);
        let vj = self.get(j);

        self.set(i, vj).set(j, vi)
    }

    fn all_rotations(&self) -> impl Iterator<Item = Point> {
        let mut all = Vec::with_capacity(24);

        for axis in 0..3 {
            for signal in [-1, 1] {
                let (axis1, axis2) = ((axis + 1) % 3, (axis + 2) % 3);
                let start = self
                    .update(axis, |v| signal * v);

                all.push(start.clone());

                let p = start
                    .update(axis1, |v| -v)
                    .update(axis2, |v| -v);

                all.push(p);

                let p = start
                    .swap(axis1, axis2)
                    .update(axis1, |v| -v);

                all.push(p.clone());

                let p = p.update(axis2, |v| -v);

                all.push(p);
            }
        }

        all.into_iter()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        Display::fmt(&self.0, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.1, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.2, f)?;
        f.write_char(')')
    }
}

struct View {
    points: Vec<(i64, i64, i64)>,
}

impl View {
    fn new() -> View {
        View {
            points: Vec::with_capacity(64)
        }
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let point = Point(1, 2, 3);

    for points in point.all_rotations() {
        println!("{}", points);
    }

    /*
    for line in reader.lines() {
        let line = line?;

    }
     */

    Ok(())
}
