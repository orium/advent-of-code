use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/05");

fn sort(rules: &HashSet<(u64, u64)>, seq: &[u64]) -> Vec<u64> {
    let mut seq = seq.to_vec();

    seq.sort_by(|&a, &b| {
        match (rules.contains(&(a, b)), rules.contains(&(b, a))) {
            // The sort is stable so if no rule is violated we keep the order.
            (false, false) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) => unreachable!(),
        }
    });

    seq
}

fn main() {
    let (rules, orders) = INPUT.split_once("\n\n").unwrap();

    let rules: HashSet<(u64, u64)> =
        rules.lines().map(|l| scan_fmt!(l, "{}|{}", u64, u64).unwrap()).collect();

    let orders =
        orders.lines().map(|order| order.split(',').map(|s| s.parse().unwrap()).collect_vec());

    let r: u64 = orders
        .map(|seq| (seq.clone(), sort(&rules, &seq)))
        .filter(|(original, sorted)| original != sorted)
        .map(|(_, sorted)| sorted[sorted.len() / 2])
        .sum();

    println!("{r}");
}
