use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("../../inputs/09");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn add(self, (dx, dy): (isize, isize)) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }

    fn sub(&self, point: &Point) -> (isize, isize) {
        (self.x - point.x, self.y - point.y)
    }
}

fn vector_from_direction(direction: char) -> (isize, isize) {
    match direction {
        'L' => (-1, 0),
        'R' => (1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => unreachable!(),
    }
}

fn tail_move_vector(head: Point, tail: Point) -> (isize, isize) {
    match head.sub(&tail) {
        (x, y) if x.abs() > 1 || y.abs() > 1 => (x.signum(), y.signum()),
        (_, _) => (0, 0),
    }
}

#[derive(Clone)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn new(n: usize) -> Rope {
        Rope { knots: vec![Point::new(0, 0); n] }
    }

    fn do_move(&mut self, direction: char) {
        let mut vector = vector_from_direction(direction);

        self.knots[0] = self.knots[0].add(vector);

        for i in 1..self.knots.len() {
            vector = tail_move_vector(self.knots[i - 1], self.knots[i]);
            self.knots[i] = self.knots[i].add(vector);
        }
    }

    fn tail(&self) -> Point {
        *self.knots.last().unwrap()
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..=5).rev() {
            for x in 0..=5 {
                match self.knots.iter().find_position(|p| p.x == x && p.y == y) {
                    None => f.write_char('.')?,
                    Some((i, _)) => f.write_fmt(format_args!("{}", i))?,
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn main() {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope: Rope = Rope::new(10);

    visited.insert(rope.tail());

    for line in INPUT.lines() {
        let (d, n) = scan_fmt!(line, "{} {}", char, usize).unwrap();

        println!("{} {}", d, n);

        for _ in 0..n {
            rope.do_move(d);
            visited.insert(rope.tail());
            println!("{}\n", rope);
        }
    }

    println!("{}", visited.len());
}
