use std::collections::BTreeSet;
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/03");

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn main(){
    let mut total: u32 = 0;

    for (a, b, c) in INPUT.lines().tuples() {
        let (a, b, c): (BTreeSet<char>, BTreeSet<char>, BTreeSet<char>) = (a.chars().collect(), b.chars().collect(), c.chars().collect());
        let intersection: BTreeSet<char> = a.intersection(&b).copied().collect::<BTreeSet<char>>().intersection(&c).copied().collect();

        total += intersection.into_iter().map(priority).sum::<u32>();
    }

    println!("{}", total);
}
