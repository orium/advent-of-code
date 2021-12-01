use std::io;
use std::io::{BufRead, BufReader, Read};

fn read_and_compute<R: Read>(reader: R) -> io::Result<u32> {
    let reader = BufReader::new(reader);
    let mut prev = None;
    let mut count = 0;

    for line in reader.lines() {
        let v: i64 = line?.parse().unwrap();

        if let Some(p) = prev {
            count += (v > p) as u32;
        }

        prev = Some(v);
    }

    Ok(count)
}

fn main() -> io::Result<()> {
    let r = read_and_compute(io::stdin())?;

    println!("{}", r);

    Ok(())
}
