use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/05");

fn read_input() -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ids) = INPUT.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|l| scan_fmt!(l, "{}-{}", u64, u64).unwrap())
        .map(|(s, e)| s..=e)
        .collect_vec();
    let ids = ids.lines().map(|l| l.parse::<u64>().unwrap()).collect_vec();

    (ranges, ids)
}

fn main() {
    let (ranges, ids) = read_input();

    let count = ids.into_iter().filter(|id| ranges.iter().any(|range| range.contains(id))).count();

    println!("{count}");
}
