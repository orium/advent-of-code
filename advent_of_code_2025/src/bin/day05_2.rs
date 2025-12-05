use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/05");

fn read_input() -> Vec<RangeInclusive<u64>> {
    let (ranges, _) = INPUT.split_once("\n\n").unwrap();

    ranges
        .lines()
        .map(|l| scan_fmt!(l, "{}-{}", u64, u64).unwrap())
        .map(|(s, e)| s..=e)
        .collect_vec()
}

fn union(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    //
    // a |----|
    //    b |----|
    //
    //    a |----|
    // b |----|
    if a.contains(b.start()) || b.contains(a.start()) {
        Some(min(*a.start(), *b.start())..=max(*a.end(), *b.end()))
    } else {
        None
    }
}

fn main() {
    let mut ranges = read_input();

    ranges.sort_by_key(|r| *r.start());

    let mut new_ranges = Vec::new();

    for range in ranges {
        match new_ranges.last_mut() {
            None => new_ranges.push(range),
            Some(last) => match union(&range, last) {
                None => new_ranges.push(range),
                Some(range_union) => {
                    *last = range_union;
                }
            },
        }
    }

    let r: u64 = new_ranges.into_iter().map(|r| r.end() - r.start() + 1).sum();

    println!("{r}");
}
