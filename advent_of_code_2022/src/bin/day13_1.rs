use std::cmp::Ordering;
use std::iter::Peekable;
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/13");

#[derive(Debug, Eq, PartialEq, Clone)]
enum Expr {
    Num(u64),
    List(Vec<Expr>),
}

impl Expr {
    fn list_singleton(expr: Expr) -> Expr {
        Expr::List(vec![expr])
    }

    fn parse(s: &str) -> Expr {
        Expr::parse_expr(&mut s.chars().peekable())
    }

    fn parse_expr(s: &mut Peekable<impl Iterator<Item=char>>) -> Expr {
        match s.peek().unwrap() {
            '[' => Expr::parse_list(s),
            _ => Expr::parse_num(s),
        }
    }

    fn parse_list(s: &mut Peekable<impl Iterator<Item=char>>) -> Expr {
        let mut v: Vec<Expr> = Vec::new();

        assert!(s.next().unwrap() == '[');

        while *s.peek().unwrap() != ']' {
            v.push(Expr::parse_expr(s));

            if *s.peek().unwrap() == ',' {
                s.next();
            }
        }

        assert!(s.next().unwrap() == ']');

        Expr::List(v)
    }

    fn parse_num(s: &mut Peekable<impl Iterator<Item=char>>) -> Expr {
        let num = s
            .peeking_take_while(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as u64)
            .fold(0, |n, d| n * 10 + d);

        Expr::Num(num)
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expr::Num(l), Expr::Num(r)) => l.cmp(r),
            (Expr::List(l), Expr::List(r)) => {
                for (lv, rv) in l.iter().zip(r.iter()) {
                    let ord_v = lv.cmp(rv);

                    if ord_v != Ordering::Equal {
                        return ord_v;
                    }
                }

                l.len().cmp(&r.len())
            },

            (l@Expr::Num(_), r@Expr::List(_)) => Expr::list_singleton(l.clone()).cmp(r),
            (l@Expr::List(_), r@Expr::Num(_)) => l.cmp(&Expr::list_singleton(r.clone())),
        }
    }
}

fn main() {
    let mut sum = 0;

    for (i, pairs_lines) in INPUT.split("\n\n").enumerate() {
        let (first, second) = pairs_lines.split_once('\n').unwrap();
        let (first, second) = (Expr::parse(first), Expr::parse(second));

        match first.cmp(&second) {
            Ordering::Less => {
                println!("ok");
                sum += i + 1;
            },
            Ordering::Equal => unreachable!(),
            Ordering::Greater => println!("not ok"),
        }
    }

    println!("{}", sum);
}
