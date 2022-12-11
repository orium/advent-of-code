use itertools::Itertools;
use num::integer::lcm;
use scan_fmt::scan_fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/11");

#[derive(Debug)]
enum Operand {
    Old,
    Num(usize),
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => Ok(Operand::Num(s.parse().map_err(|_| ())?)),
        }
    }
}

impl Operand {
    fn get(&self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Num(v) => *v,
        }
    }
}

#[derive(Debug)]
struct Operation {
    left: Operand,
    op: char,
    right: Operand,
}

impl Operation {
    fn calc(&self, old: usize) -> usize {
        match self.op {
            '*' => self.left.get(old) * self.right.get(old),
            '+' => self.left.get(old) + self.right.get(old),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisor: usize,
    throw_true: usize,
    throw_false: usize,

    inspections: usize,
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_str in INPUT.split("\n\n") {
        let mut iter = monkey_str.lines();

        iter.next();

        let items_str =
            scan_fmt!(iter.next().unwrap(), "  Starting items: {/.*/}", String).unwrap();
        let (op_l_str, op, op_r_str) =
            scan_fmt!(iter.next().unwrap(), "  Operation: new = {} {} {}", String, char, String)
                .unwrap();
        let test_divisor =
            scan_fmt!(iter.next().unwrap(), "  Test: divisible by {}", usize).unwrap();
        let throw_true =
            scan_fmt!(iter.next().unwrap(), "    If true: throw to monkey {}", usize).unwrap();
        let throw_false =
            scan_fmt!(iter.next().unwrap(), "    If false: throw to monkey {}", usize).unwrap();

        let items = items_str.split(", ").map(|s| s.parse().unwrap()).collect_vec();
        let operation = Operation {
            left: Operand::from_str(&op_l_str).unwrap(),
            op,
            right: Operand::from_str(&op_r_str).unwrap(),
        };

        let monkey = Monkey {
            items,
            operation,
            test_divisor,
            throw_true,
            throw_false,

            inspections: 0,
        };

        monkeys.push(monkey);
    }

    let lcm = monkeys.iter().map(|m| m.test_divisor).reduce(|a, b| lcm(a, b)).unwrap();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.drain(..).collect_vec();

            monkeys[i].inspections += items.len();

            for item in items {
                let v = monkeys[i].operation.calc(item) % lcm;
                let throw_true = monkeys[i].throw_true;
                let throw_false = monkeys[i].throw_false;

                match v % monkeys[i].test_divisor {
                    0 => monkeys[throw_true].items.push(v),
                    _ => monkeys[throw_false].items.push(v),
                }
            }
        }
    }

    println!("{}", monkeys.iter().map(|m| m.inspections).sorted().rev().take(2).product::<usize>());
}
