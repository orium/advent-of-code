use aoc_utils::MyItertools;
use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/05");

fn is_correct(rules: &[(u64, u64)], order: &[u64]) -> bool {
    rules.iter().all(|(l, h)| {
        order
            .iter()
            .position_of(l)
            .zip(order.iter().position_of(h))
            .map_or(true, |(il, ih)| il < ih)
    })
}

fn main() {
    let (rules, orders) = INPUT.split_once("\n\n").unwrap();

    let rules: Vec<(u64, u64)> =
        rules.lines().map(|l| scan_fmt!(l, "{}|{}", u64, u64).unwrap()).collect();

    let orders =
        orders.lines().map(|order| order.split(',').map(|s| s.parse().unwrap()).collect_vec());

    let r: u64 = orders.filter(|order| is_correct(&rules, order)).map(|s| s[s.len() / 2]).sum();

    println!("{r}");
}
