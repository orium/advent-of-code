use scan_fmt::scan_fmt;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/04");

fn parse(line: &str) -> (HashSet<u64>, HashSet<u64>) {
    let (w, p) = scan_fmt!(line, "Card {*d}: {/[0-9 ]*/} | {/[0-9 ]*$/}", String, String).unwrap();

    let w = w.split(' ').filter_map(|s| s.parse().ok()).collect();
    let p = p.split(' ').filter_map(|s| s.parse().ok()).collect();

    (w, p)
}

fn main() {
    let r: u64 = INPUT
        .lines()
        .map(parse)
        .map(|(w, p)| p.intersection(&w).count() as u32)
        .filter(|c| *c > 0)
        .map(|c| 2_u64.pow(c - 1))
        .sum();

    println!("{r}");
}
