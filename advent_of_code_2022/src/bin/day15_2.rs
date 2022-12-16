use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/15");

fn distance((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs()
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    reach: i64,
}

impl Sensor {
    fn visibility_on_x(&self, y: i64) -> RangeInclusive<i64> {
        let dist_y = (self.y - y).abs();

        (self.x - self.reach + dist_y)..=(self.x + self.reach - dist_y)
    }
}

fn main() {
    let mut beacons: HashSet<(i64, i64)> = HashSet::new();
    let mut sensors: Vec<Sensor> = Vec::new();

    for line in INPUT.lines() {
        let (sx, sy, bx, by) = scan_fmt!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        let sensor = Sensor { x: sx, y: sy, reach: distance((sx, sy), (bx, by)) };

        beacons.insert((bx, by));
        sensors.push(sensor);
    }

    const X_MIN_COORD: i64 = 0;
    const X_MAX_COORD: i64 = 4_000_000;
    const Y_MIN_COORD: i64 = 0;
    const Y_MAX_COORD: i64 = 4_000_000;

    for y in Y_MIN_COORD..=Y_MAX_COORD {
        let ranges = sensors.iter().map(|s| s.visibility_on_x(y)).collect_vec();
        let mut candidates = ranges
            .iter()
            .filter(|r| !r.is_empty())
            .flat_map(|r| [r.start() - 1, r.end() + 1].into_iter())
            .filter(|&x| X_MIN_COORD <= x && x <= X_MAX_COORD);
        let dark = candidates.find(|x| ranges.iter().all(|r| !r.contains(x)));

        if let Some(x) = dark {
            println!("{:?}", (x, y));
            println!("{}", x * 4_000_000 + y);
            break;
        }
    }
}
