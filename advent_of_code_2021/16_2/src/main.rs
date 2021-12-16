use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equals,
}

#[derive(Debug)]
enum Expr {
    Num(u64),
    Op(Operator, Vec<Packet>)
}

#[derive(Debug)]
struct Packet {
    version: u64,
    expr: Expr,
}

fn parse_hex_to_bin(data: &str) -> Vec<u8> {
    let mut bin: Vec<u8> = Vec::with_capacity(4 * data.len());

    for c in data.chars() {
        let d = c.to_digit(16).unwrap();

        bin.push(((d & 0b1000) >> 3) as u8);
        bin.push(((d & 0b0100) >> 2) as u8);
        bin.push(((d & 0b0010) >> 1) as u8);
        bin.push(((d & 0b0001) >> 0) as u8);
    }

    bin
}

fn get_bin_val(bin: &[u8], index: usize, length: usize) -> u64 {
    let mut r = 0;

    for i in 0..length {
        r = (r << 1) | (bin[index + i] as u64);
    }

    r
}

fn parse_packet(bin: &[u8]) -> (usize, Packet) {
    let version = get_bin_val(&bin, 0, 3);
    let type_id = get_bin_val(&bin, 3, 3);

    match type_id {
        4 => {
            let mut num = 0;
            let mut base = 6;

            loop {
                let v = get_bin_val(&bin, base, 5);
                let is_last = (v & 0b1_0000) == 0;
                let v = v & 0b1111;

                num = (num << 4) | v;

                base += 5;

                if is_last {
                    break;
                }
            }

            (base, Packet { version, expr: Expr::Num(num) })
        },
        op => {
            let length_type = get_bin_val(&bin, 6, 1);

            let op = match op {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Min,
                3 => Operator::Max,
                5 => Operator::GreaterThan,
                6 => Operator::LessThan,
                7 => Operator::Equals,
                _ => unreachable!(),
            };

            match length_type {
                0 => {
                    let total_subpacket_length = get_bin_val(&bin, 7, 15) as usize;
                    let start = 7 + 15;
                    let mut consumed: usize = 0;
                    let mut operands: Vec<Packet> = Vec::with_capacity(4);

                    while consumed < total_subpacket_length {
                        let (packet_consumed, packet) = parse_packet(&bin[start + consumed..]);
                        operands.push(packet);
                        consumed += packet_consumed;
                    }

                    (start + consumed, Packet { version, expr: Expr::Op(op, operands) })
                },
                1 => {
                    let packet_count = get_bin_val(&bin, 7, 11);
                    let start = 7 + 11;
                    let mut consumed: usize = 0;
                    let mut operands: Vec<Packet> = Vec::with_capacity(4);

                    for _ in 0..packet_count {
                        let (packet_consumed, packet) = parse_packet(&bin[start + consumed..]);
                        operands.push(packet);
                        consumed += packet_consumed;
                    }

                    (start + consumed, Packet { version, expr: Expr::Op(op, operands) })
                },
                _ => unreachable!(),
            }
        },
    }
}

fn parse(data: &str) -> Packet {
    let bin: Vec<u8> = parse_hex_to_bin(data);

    parse_packet(&bin).1
}

fn eval(packet: &Packet) -> u64 {
    match packet.expr {
        Expr::Num(n) => n,
        Expr::Op(operand, ref packets) => {
            let mut results = packets.iter().map(eval);

            match operand {
                Operator::Sum => results.sum(),
                Operator::Product => results.product(),
                Operator::Min => results.min().unwrap(),
                Operator::Max => results.max().unwrap(),
                Operator::GreaterThan => (results.next().unwrap() > results.next().unwrap()) as u64,
                Operator::LessThan => (results.next().unwrap() < results.next().unwrap()) as u64,
                Operator::Equals => (results.next().unwrap() == results.next().unwrap()) as u64,
            }
        },
    }
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());

    for line in reader.lines() {
        let line = line.unwrap();
        let packet = parse(&line);

        // println!("{:#?}", packet);
        println!("{}", eval(&packet));
    }

    Ok(())
}
