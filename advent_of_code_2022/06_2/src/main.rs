use std::collections::HashSet;
use itertools::Itertools;

const INPUT: &str = include_str!("../input");

fn main() {
    let chars = INPUT.chars().collect_vec();

    let index =
        chars
            .windows(14)
            .enumerate()
            .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == 14)
            .map(|(i, _)| i)
            .unwrap();

    println!("{}", index + 14);
}
