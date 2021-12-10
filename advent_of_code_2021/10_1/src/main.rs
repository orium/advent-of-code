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
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Status {
    Ok,
    Corrupted { expected: char, got: char },
    Incomplete,
    Invalid,
}

fn check(line: &str) -> Status {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        } else {
            let expected = close_to_open(c);

            match stack.pop() {
                None => {
                    return Status::Invalid;
                }
                Some(sc) if sc == expected => (),
                Some(sc) => {
                    return Status::Corrupted { expected: open_to_close(sc), got: c };
                }
            }
        }
    }

    match stack.is_empty() {
        true => Status::Ok,
        false => Status::Incomplete,
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut sum = 0;

    for line in reader.lines() {
        let line: String = line?.parse().unwrap();
        let status = check(&line);

        println!("{}: {:?}", line, status);

        if let Status::Corrupted { got, .. } = status {
            println!("    points: {}", points(got));
            sum += points(got);
        }
    }

    println!("{}", sum);

    Ok(())
}
