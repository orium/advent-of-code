use itertools::Itertools;
use num::Rational;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, Read};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn distance_squared(&self, other: Point) -> usize {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;

        (dx * dx + dy * dy) as usize
    }

    fn angle(&self, other: Point) -> f64 {
        ((other.y as isize - self.y as isize) as f64)
            .atan2((other.x as isize - self.x as isize) as f64)
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Line {
    Normal { slope: Rational, b: Rational },
    Vertical,
}

impl Line {
    fn from_points(p1: Point, p2: Point) -> Line {
        if p2.x as isize - p1.x as isize == 0 {
            Line::Vertical
        } else {
            let slope = Rational::from_integer(p2.y as isize - p1.y as isize)
                / Rational::from_integer(p2.x as isize - p1.x as isize);
            let b = Rational::from_integer(p1.y as isize) - slope * (p1.x as isize);
            Line::Normal { slope, b }
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    LeftOrBelow,
    RightOrAbove,
}

impl Direction {
    fn from_points(reference: Point, p: Point) -> Direction {
        match reference.x.cmp(&p.x) {
            Ordering::Less => Direction::RightOrAbove,
            Ordering::Equal => match reference.y.cmp(&p.y) {
                Ordering::Less => Direction::RightOrAbove,
                Ordering::Equal => panic!("Same point"),
                Ordering::Greater => Direction::LeftOrBelow,
            },
            Ordering::Greater => Direction::LeftOrBelow,
        }
    }
}

fn read_asteroids<R: Read>(reader: R) -> io::Result<Vec<Point>> {
    let reader = BufReader::new(reader);
    let mut asteroids: Vec<Point> = Vec::new();

    for (line, y) in reader.lines().zip(0..) {
        let line = line?;

        if line == "END" {
            break;
        }

        for (c, x) in line.chars().zip(0..) {
            if c == '#' {
                asteroids.push(Point::new(x, y));
            }
        }
    }

    Ok(asteroids)
}

fn get_asteroids_by_line(
    station: Point,
    mut asteroids: impl Iterator<Item = Point>,
) -> Vec<Vec<Point>> {
    let mut by_line: HashMap<(Line, Direction), Vec<Point>> = HashMap::new();

    while let Some(asteroid) = asteroids.next() {
        let line = Line::from_points(station, asteroid);
        let direction = Direction::from_points(station, asteroid);
        by_line.entry((line, direction)).or_default().push(asteroid);
    }

    by_line.into_iter().map(|(_, v)| v).collect_vec()
}

fn sort_by_angle(station: Point, by_line: &mut Vec<Vec<Point>>) {
    by_line.sort_by_key(|line| {
        let angle = station.angle(line[0]);
        let angle = if angle < 0.0 { 2.0 * std::f64::consts::PI + angle } else { angle };

        let angle = angle - 3.0 * std::f64::consts::FRAC_PI_2;
        let angle = if angle < 0.0 { 2.0 * std::f64::consts::PI + angle } else { angle };

        assert!(angle >= 0.0);
        assert!(angle < 2.0 * std::f64::consts::PI);

        (10_000_000.0 * angle) as usize
    });
}

fn sort_by_distance(station: Point, by_line: &mut Vec<Vec<Point>>) {
    for asteroids in by_line.iter_mut() {
        asteroids.sort_by_key(|&a| station.distance_squared(a));
        asteroids.reverse();
    }
}

fn main() -> io::Result<()> {
    let asteroids = read_asteroids(io::stdin())?;
    let station: Point = Point::new(13, 17);

    let mut asteroids_by_line: Vec<Vec<Point>> =
        get_asteroids_by_line(station, asteroids.into_iter().filter(|&p| p != station));

    sort_by_angle(station, &mut asteroids_by_line);
    sort_by_distance(station, &mut asteroids_by_line);

    let mut count = 0;
    let mut last_asteroid: Option<Point> = None;

    'outer: loop {
        for asteroids in asteroids_by_line.iter_mut() {
            match asteroids.pop() {
                None => (),
                Some(a) => {
                    last_asteroid = Some(a);
                }
            }

            count += 1;

            if count == 200 {
                break 'outer;
            }
        }
    }

    println!("{:?}", 100 * last_asteroid.unwrap().x + last_asteroid.unwrap().y);

    Ok(())
}
