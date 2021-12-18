use std::fmt::{Display, Formatter, Write};
use std::io;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;

#[derive(Clone, Debug)]
enum Expr {
    Pair(Pair),
    Num(u64),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Pair(p) => Display::fmt(p, f),
            Expr::Num(n) => Display::fmt(n, f),
        }
    }
}

#[derive(Clone, Debug)]
struct Pair {
    left: Box<Expr>,
    right: Box<Expr>,
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        Display::fmt(&self.left, f)?;
        f.write_char(',')?;
        Display::fmt(&self.right, f)?;
        f.write_char(']')
    }
}

fn expect(src: &mut Peekable<std::str::Chars>, ch: char) {
    let next = src.next();
    assert_eq!(next, Some(ch), "expected {}, got {}", ch, next.unwrap_or('?'));
}

fn parse_num(src: &mut Peekable<std::str::Chars>) -> u64 {
    let c = src.next().expect("unexpected end of expression");

    c.to_digit(10).expect("expected digit") as u64
}

fn parse_expr(src: &mut Peekable<std::str::Chars>) -> Expr {
    match src.peek().expect("unexpected end of expression").is_ascii_digit() {
        true => Expr::Num(parse_num(src)),
        false => Expr::Pair(parse_pair(src)),
    }
}

fn parse_pair(src: &mut Peekable<std::str::Chars>) -> Pair {
    expect(src, '[');
    let left = parse_expr(src);
    expect(src, ',');
    let right = parse_expr(src);
    expect(src, ']');

    Pair { left: Box::new(left), right: Box::new(right) }
}

#[derive(Debug)]
struct Todo {
    increase_left: u64,
    increase_right: u64,
    num_index: usize,
}

fn reduce_explode(pair: &Pair, depth: usize, num_count: &mut usize) -> Option<Todo> {
    if depth == 4 {
        if let (Expr::Num(l), Expr::Num(r)) = (pair.left.as_ref(), pair.right.as_ref()) {
            return Some(Todo { increase_left: *l, increase_right: *r, num_index: *num_count });
        }
    }

    match *pair.left {
        Expr::Pair(ref l) => {
            if let Some(todo) = reduce_explode(l, depth + 1, num_count) {
                return Some(todo);
            }
        }
        Expr::Num(_) => *num_count += 1,
    }

    match *pair.right {
        Expr::Pair(ref r) => {
            if let Some(todo) = reduce_explode(r, depth + 1, num_count) {
                return Some(todo);
            }
        }
        Expr::Num(_) => *num_count += 1,
    }

    None
}

fn apply_todo(pair: &mut Pair, todo: &Todo, num_count: &mut usize) -> bool {
    if *num_count == todo.num_index {
        if let (Expr::Num(_), Expr::Num(_)) = (pair.left.as_ref(), pair.right.as_ref()) {
            *num_count += 2;
            return true;
        }
    }

    match *pair.left {
        Expr::Pair(ref mut l) => {
            if apply_todo(l, todo, num_count) {
                *pair.left = Expr::Num(0);
            }
        }
        Expr::Num(n) => {
            *num_count += 1;
            if *num_count == todo.num_index {
                *pair.left = Expr::Num(n + todo.increase_left);
            } else if *num_count == todo.num_index + 3 {
                *pair.left = Expr::Num(n + todo.increase_right);
            }
        },
    }

    match *pair.right {
        Expr::Pair(ref mut r) => {
            if apply_todo(r, todo, num_count) {
                *pair.right = Expr::Num(0);
            }
        }
        Expr::Num(n) => {
            *num_count += 1;
            if *num_count == todo.num_index {
                *pair.right = Expr::Num(n + todo.increase_left);
            } else if *num_count == todo.num_index + 3 {
                *pair.right = Expr::Num(n + todo.increase_right);
            }
        },
    }

    false
}

fn split(n: u64) -> Pair {
    Pair {
        left: Box::new(Expr::Num(n / 2)),
        right: Box::new(Expr::Num(n / 2 + n % 2)),
    }
}

fn reduce_split(pair: &mut Pair) -> bool {
    match *pair.left {
        Expr::Pair(ref mut l) => {
            if reduce_split(l) {
                return true;
            }
        }
        Expr::Num(n) if n >= 10 => {
            *pair.left = Expr::Pair(split(n));
            return true;
        },
        Expr::Num(_) => (),
    }

    match *pair.right {
        Expr::Pair(ref mut r) => {
            if reduce_split(r) {
                return true;
            }
        }
        Expr::Num(n) if n >= 10 => {
            *pair.right = Expr::Pair(split(n));
            return true;
        },
        Expr::Num(_) => (),
    }

    false
}

fn reduce(pair: &mut Pair) {
    let mut num_count;

    loop {
        num_count = 0;

        if let Some(todo) = reduce_explode(pair, 0, &mut num_count) {
            num_count = 0;
            apply_todo(pair, &todo, &mut num_count);
            // println!("exploded to: {}", pair);
            continue;
        }

        if reduce_split(pair) {
            // println!("splitted to: {}", pair);
            continue;
        }

        break;
    }
}

fn add(left: Pair, right: Pair) -> Pair {
    let mut sum = Pair { left: Box::new(Expr::Pair(left)), right: Box::new(Expr::Pair(right)) };

    reduce(&mut sum);

    sum
}

fn magnitude(pair: &Pair) -> u64 {
    let lm = match *pair.left {
        Expr::Pair(ref l) => magnitude(l),
        Expr::Num(n) => n
    };

    let rm = match *pair.right {
        Expr::Pair(ref r) => magnitude(r),
        Expr::Num(n) => n,
    };

    3 * lm + 2 * rm
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut pairs: Vec<Pair> = Vec::with_capacity(128);

    for line in reader.lines() {
        let line = line?;
        let mut char_iter = line.chars().peekable();
        let pair = parse_pair(&mut char_iter);

        pairs.push(pair);
    }

    let mut max: u64 = 0;

    for i in 0..pairs.len() {
        for j in (0..pairs.len()).filter(|j| *j != i) {
            let sum = add(pairs[i].clone(), pairs[j].clone());

            max = u64::max(max, magnitude(&sum));
        }
    }

    println!("{}", max);

    Ok(())
}
