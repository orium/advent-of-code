use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut max: usize = 0;
    let mut current: usize = 0;

    for line in reader.lines() {
        let line = line?;

        match line.parse::<usize>() {
            Ok(num) => current += num,
            Err(_) => {
                max = max.max(current);
                current = 0;
            }
        }
    }

    assert_eq!(current, 0);

    println!("{}", max);

    Ok(())
}
