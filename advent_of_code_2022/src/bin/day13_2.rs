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
    let divs = [
        Expr::list_singleton(Expr::list_singleton(Expr::Num(2))),
        Expr::list_singleton(Expr::list_singleton(Expr::Num(6))),
    ];
    let mut packets = Vec::from(divs.clone());

    for line in INPUT.lines() {
        if !line.is_empty() {
            packets.push(Expr::parse(line));
        }
    }

    packets.sort();

    let r: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, e)| divs.contains(e))
        .map(|(i, _)| i + 1)
        .product();

    println!("{}", r);
}
