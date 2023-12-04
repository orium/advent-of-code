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
    let matches: Vec<usize> =
        INPUT.lines().map(parse).map(|(w, p)| p.intersection(&w).count()).collect();

    let mut counts: Vec<u64> = vec![1; matches.len()];

    for (i, matches) in matches.into_iter().enumerate() {
        for j in 0..matches {
            counts[i + j + 1] += counts[i];
        }
    }

    println!("{}", counts.iter().sum::<u64>());
}
