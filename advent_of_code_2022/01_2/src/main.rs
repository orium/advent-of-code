use std::io;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut cals: BinaryHeap<usize> = BinaryHeap::new();
    let mut current: usize = 0;

    for line in reader.lines() {
        let line = line?;

        match line.parse::<usize>() {
            Ok(num) => current += num,
            Err(_) => {
                cals.push(current);
                current = 0;
            }
        }
    }

    assert_eq!(current, 0);

    println!("{}", (0..3).filter_map(|_| cals.pop()).sum::<usize>());

    Ok(())
}
