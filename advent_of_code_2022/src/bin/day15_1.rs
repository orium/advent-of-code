use scan_fmt::scan_fmt;
use std::collections::HashSet;

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
    fn within_reach(&self, (x, y): (i64, i64)) -> bool {
        distance((self.x, self.y), (x, y)) <= self.reach
    }
}

fn can_have_beacon(
    (x, y): (i64, i64),
    beacons: &HashSet<(i64, i64)>,
    sensors: &Vec<Sensor>,
) -> bool {
    beacons.contains(&(x, y)) || sensors.iter().all(|s| !s.within_reach((x, y)))
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

        println!("{:?}", sensor);

        beacons.insert((bx, by));
        sensors.push(sensor);
    }

    let min_x: i64 = sensors.iter().map(|s| s.x - s.reach).min().unwrap();
    let max_x: i64 = sensors.iter().map(|s| s.x + s.reach).max().unwrap();

    const Y: i64 = 2000000;

    let r = (min_x..=max_x).filter(|x| !can_have_beacon((*x, Y), &beacons, &sensors)).count();

    println!("{}", r);
}
