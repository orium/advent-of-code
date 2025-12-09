use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/09");

#[derive(Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn rectangle_area_inclusive(self, other: Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn main() {
    let points: Vec<Point> = INPUT
        .lines()
        .map(|line| scan_fmt!(line, "{},{}", u64, u64).unwrap())
        .map(|(x, y)| Point { x, y })
        .collect();

    let r = points
        .iter()
        .tuple_combinations()
        .map(|(p, q)| p.rectangle_area_inclusive(*q))
        .max()
        .unwrap_or_default();

    println!("{r}");
}
