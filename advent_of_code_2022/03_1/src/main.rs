use std::collections::BTreeSet;
use std::io;
use std::io::{BufRead, BufReader};

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

    for line in reader.lines() {
        let line = line?;
        let (a, b) = line.split_at(line.len()/2);
        let (a, b) : (BTreeSet<char>, BTreeSet<char>) = (a.chars().collect(), b.chars().collect());

        total += a.intersection(&b).copied().map(priority).sum::<u32>();
    }

    println!("{}", total);

    Ok(())
}
