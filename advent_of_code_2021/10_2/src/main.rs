use std::io;
use std::io::{BufRead, BufReader};

fn is_open(c: char) -> bool {
    "([{<".contains(c)
}

fn open_to_close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn close_to_open(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn points(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Error {
    Corrupted { expected: char, got: char },
    Incomplete { fix: String },
    Invalid,
}

fn check(line: &str) -> Result<(), Error> {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.chars() {
        match is_open(c) {
            true => stack.push(c),
            false => {
                let need = close_to_open(c);

                match stack.pop() {
                    None => return Err(Error::Invalid),
                    Some(sc) if sc == need => (),
                    Some(sc) => return Err(Error::Corrupted { expected: open_to_close(sc), got: c }),
                }
            }
        }
    }

    match stack.is_empty() {
        true => Ok(()),
        false => Err(Error::Incomplete { fix: stack.iter().rev().copied().map(open_to_close).collect() }),
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut all_points = Vec::new();

    for line in reader.lines() {
        let line: String = line?.parse().unwrap();
        let status = check(&line);

        println!("{}: {:?}", line, status);

        if let Err(Error::Incomplete { fix }) = status {
            println!("    fix: {}", fix);

            let mut p = 0;

            for c in fix.chars() {
                p = 5 * p + points(c) as u128;
            }

            all_points.push(p);
        }
    }

    all_points.sort();

    println!("{}", all_points[all_points.len()/2]);

    Ok(())
}
