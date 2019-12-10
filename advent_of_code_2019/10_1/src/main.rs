use num::Rational;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::{BufRead, BufReader, Read};

#[derive(Hash, PartialEq, Eq)]
enum Line {
    Normal { slope: Rational, b: Rational },
    Vertical,
}

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    LeftOrBelow,
    RightOrAbove,
}

impl Direction {
    fn from_points(reference: (usize, usize), p: (usize, usize)) -> Direction {
        match reference.0.cmp(&p.0) {
            Ordering::Less => Direction::RightOrAbove,
            Ordering::Equal => match reference.1.cmp(&p.1) {
                Ordering::Less => Direction::RightOrAbove,
                Ordering::Equal => panic!("Same point"),
                Ordering::Greater => Direction::LeftOrBelow,
            },
            Ordering::Greater => Direction::LeftOrBelow,
        }
    }
}

impl Line {
    fn from_points(p1: (usize, usize), p2: (usize, usize)) -> Line {
        if p2.0 as isize - p1.0 as isize == 0 {
            Line::Vertical
        } else {
            let slope = Rational::from_integer(p2.1 as isize - p1.1 as isize)
                / Rational::from_integer(p2.0 as isize - p1.0 as isize);
            let b = Rational::from_integer(p1.1 as isize) - slope * (p1.0 as isize);
            Line::Normal { slope, b }
        }
    }
}

fn read_asteroids<R: Read>(reader: R) -> io::Result<Vec<(usize, usize)>> {
    let reader = BufReader::new(reader);
    let mut asteroids: Vec<(usize, usize)> = Vec::new();

    for (line, y) in reader.lines().zip(0..) {
        let line = line?;

        for (c, x) in line.chars().zip(0..) {
            if c == '#' {
                asteroids.push((x, y));
            }
        }
    }

    Ok(asteroids)
}

fn visible_from_station(station: (usize, usize), asteroids: &Vec<(usize, usize)>) -> usize {
    let mut lines: HashSet<(Line, Direction)> = HashSet::new();

    for &asteroid in asteroids {
        if asteroid != station {
            let direction = Direction::from_points(station, asteroid);
            lines.insert((Line::from_points(station, asteroid), direction));
        }
    }

    lines.len()
}

fn main() -> io::Result<()> {
    let asteroids = read_asteroids(io::stdin())?;
    let mut most_visible = 0;
    let mut best_asteroid = None;

    for &a in asteroids.iter() {
        let visible = visible_from_station(a, &asteroids);
        if visible > most_visible {
            most_visible = visible;
            best_asteroid = Some(a);
        }
    }

    println!("{} at {:?}", most_visible, best_asteroid.unwrap());

    Ok(())
}
