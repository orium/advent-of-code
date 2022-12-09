use std::collections::HashSet;

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

fn move_segment(head: Point, tail: Point, direction: &str) -> (Point, Point) {
    let head_vector: (isize, isize) = match direction {
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => unreachable!(),
    };
    let head = head.add(head_vector);

    let tail_vector: (isize, isize) = match head.sub(&tail) {
        (2, y) => (1, y),
        (-2, y) => (-1, y),

        (x, 2) => (x, 1),
        (x, -2) => (x, -1),

        (_, _) => (0, 0),
    };

    let tail = tail.add(tail_vector);

    (head, tail)
}

fn main() {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);

    visited.insert(tail);

    for line in INPUT.lines() {
        let (d, n) = line.split_once(' ').unwrap();
        let n: usize = n.parse().unwrap();

        println!("{} {}", d, n);

        for _ in 0..n {
            (head, tail) = move_segment(head, tail, d);
            visited.insert(tail);
            println!("head {:?} tail {:?}", head, tail);
        }
    }

    println!("{}", visited.len());
}
