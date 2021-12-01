use std::collections::VecDeque;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn read_and_compute<R: Read>(reader: R) -> io::Result<u32> {
    let reader = BufReader::new(reader);
    let mut count = 0;
    let mut current = 0;
    let mut dequeue = VecDeque::with_capacity(3);

    for line in reader.lines() {
        let v: i64 = line?.parse().unwrap();

        if dequeue.len() < 3 {
            dequeue.push_back(v);
            current += v;
            continue;
        }

        let d = dequeue.pop_front().unwrap();
        dequeue.push_back(v);

        let next = current - d + v;

        count += (next > current) as u32;

        current = next;
    }

    Ok(count)
}

fn main() -> io::Result<()> {
    let r = read_and_compute(io::stdin())?;

    println!("{}", r);

    Ok(())
}
