use std::collections::BTreeSet;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut total: u32 = 0;

    for (a, b, c) in reader.lines().tuples() {
        let (a, b, c) = (a?, b?, c?);
        let (a, b, c): (BTreeSet<char>, BTreeSet<char>, BTreeSet<char>) = (a.chars().collect(), b.chars().collect(), c.chars().collect());
        let intersection: BTreeSet<char> = a.intersection(&b).copied().collect::<BTreeSet<char>>().intersection(&c).copied().collect();

        total += intersection.into_iter().map(priority).sum::<u32>();
    }

    println!("{}", total);

    Ok(())
}
