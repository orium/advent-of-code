use lazy_static::lazy_static;
use num::bigint::ToBigInt;
use num::Zero;
use num::{BigInt, ToPrimitive};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::{Index, IndexMut};

pub enum AddressingMode {
    Position,
    Immediate,
    Relative,
}

impl From<u32> for AddressingMode {
    fn from(v: u32) -> AddressingMode {
        match v {
            0 => AddressingMode::Position,
            1 => AddressingMode::Immediate,
            2 => AddressingMode::Relative,
            _ => panic!("invalid addressing mode"),
        }
    }
}

#[derive(Clone)]
pub struct Memory {
    mem: HashMap<BigInt, BigInt>,
}

impl Memory {
    pub fn load_line<R: Read>(reader: R) -> io::Result<Memory> {
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        reader.read_line(&mut line)?;

        let input = line.split(',').map(|s| s.trim().parse::<BigInt>().unwrap());
        let mut mem: HashMap<BigInt, BigInt> = HashMap::new();

        for (i, v) in (0usize..).zip(input) {
            mem.insert(i.to_bigint().unwrap(), v);
        }

        Ok(Memory { mem })
    }
}

lazy_static! {
    static ref BIGINT_ZERO: BigInt = BigInt::zero();
}

impl Index<BigInt> for Memory {
    type Output = BigInt;

    fn index(&self, index: BigInt) -> &BigInt {
        assert!(index >= BigInt::zero());
        self.mem.get(&index).unwrap_or(&BIGINT_ZERO)
    }
}

impl IndexMut<BigInt> for Memory {
    fn index_mut(&mut self, index: BigInt) -> &mut BigInt {
        assert!(index >= BigInt::zero());

        if !self.mem.contains_key(&index) {
            self.mem.insert(index.clone(), BigInt::zero());
        }

        self.mem.get_mut(&index).unwrap()
    }
}

#[derive(Clone)]
pub struct Computer {
    memory: Memory,
    program_counter: BigInt,
    relative_base: BigInt,
    input_stream: VecDeque<BigInt>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum State {
    AwaitingInput,
    Outputed(BigInt),
    Halted,
}

impl Computer {
    pub fn new(memory: Memory) -> Computer {
        Computer {
            memory,
            program_counter: BigInt::zero(),
            relative_base: BigInt::zero(),
            input_stream: VecDeque::new(),
        }
    }

    fn read_argument(&self, argument: &BigInt, mode: AddressingMode) -> BigInt {
        match mode {
            AddressingMode::Position => self.memory[argument.clone()].clone(),
            AddressingMode::Immediate => argument.clone(),
            AddressingMode::Relative => self.memory[self.relative_base.clone() + argument].clone(),
        }
    }

    fn store_result(&mut self, argument: BigInt, value: BigInt, mode: AddressingMode) {
        match mode {
            AddressingMode::Position => self.memory[argument.clone()] = value,
            AddressingMode::Immediate => panic!("Cannot store in immediate mode"),
            AddressingMode::Relative => self.memory[self.relative_base.clone() + argument] = value,
        }
    }

    pub fn enqueue_input(&mut self, v: BigInt) {
        self.input_stream.push_back(v);
    }

    pub fn run_non_blocking(&mut self) -> State {
        loop {
            let instruction: u32 = self.memory[self.program_counter.clone()].to_u32().unwrap();
            let opcode = instruction % 100;
            let address_mode = |position: u8| -> AddressingMode {
                AddressingMode::from(instruction / 10u32.pow(position as u32 + 2) % 10)
            };

            match opcode {
                1 | 2 => {
                    let v0 = self.read_argument(
                        &self.memory[self.program_counter.clone() + 1],
                        address_mode(0),
                    );
                    let v1 = self.read_argument(
                        &self.memory[self.program_counter.clone() + 2],
                        address_mode(1),
                    );

                    let result = match opcode {
                        1 => v0 + v1,
                        2 => v0 * v1,
                        _ => unreachable!(),
                    };

                    self.store_result(
                        self.memory[self.program_counter.clone() + 3].clone(),
                        result,
                        address_mode(2),
                    );

                    self.program_counter += 4;
                }
                3 => {
                    let v: BigInt = match self.input_stream.pop_front() {
                        Some(v) => v,
                        None => return State::AwaitingInput,
                    };

                    self.store_result(
                        self.memory[self.program_counter.clone() + 1].clone(),
                        v,
                        address_mode(0),
                    );

                    self.program_counter += 2;
                }
                4 => {
                    let v = self.read_argument(
                        &self.memory[self.program_counter.clone() + 1],
                        address_mode(0),
                    );
                    self.program_counter += 2;
                    return State::Outputed(v);
                }
                5 | 6 => {
                    let jump_on: bool = opcode == 5;
                    let cond = self.read_argument(
                        &self.memory[self.program_counter.clone() + 1],
                        address_mode(0),
                    );
                    let jump_to = self.read_argument(
                        &self.memory[self.program_counter.clone() + 2],
                        address_mode(1),
                    );

                    self.program_counter = match (cond != BigInt::zero()) == jump_on {
                        true => jump_to,
                        false => self.program_counter.clone() + 3,
                    };
                }
                7 | 8 => {
                    let v0 = self.read_argument(
                        &self.memory[self.program_counter.clone() + 1],
                        address_mode(0),
                    );
                    let v1 = self.read_argument(
                        &self.memory[self.program_counter.clone() + 2],
                        address_mode(1),
                    );

                    let result = match opcode {
                        7 => (v0 < v1) as u32,
                        8 => (v0 == v1) as u32,
                        _ => unreachable!(),
                    }
                    .to_bigint()
                    .unwrap();

                    self.store_result(
                        self.memory[self.program_counter.clone() + 3].clone(),
                        result,
                        address_mode(2),
                    );

                    self.program_counter += 4;
                }
                9 => {
                    let v = self.read_argument(
                        &self.memory[self.program_counter.clone() + 1],
                        address_mode(0),
                    );
                    self.relative_base += v;
                    self.program_counter += 2;
                }
                99 => return State::Halted,
                opcode => panic!("Invalid opcode {}", opcode),
            }
        }
    }

    pub fn run_to_completion<R: Read, W: Write>(&mut self, input: R, mut output: W) {
        let mut input = BufReader::new(input);

        loop {
            match self.run_non_blocking() {
                State::AwaitingInput => {
                    let mut line = String::new();
                    input.read_line(&mut line).ok().expect("Failed to read input");
                    let v: BigInt = line.trim().parse().unwrap();
                    self.enqueue_input(v);
                }
                State::Outputed(v) => {
                    output.write_fmt(format_args!("{}\n", v)).ok().expect("Failed to write output");
                }
                State::Halted => break,
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_right(self) -> Direction {
        self.turn_left().turn_left().turn_left()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    fn advance(self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position { x: self.x, y: self.y - 1 },
            Direction::East => Position { x: self.x + 1, y: self.y },
            Direction::South => Position { x: self.x, y: self.y + 1 },
            Direction::West => Position { x: self.x - 1, y: self.y },
        }
    }
}

fn run_paint_job(computer: &mut Computer) -> HashMap<Position, u8> {
    let mut direction = Direction::North;
    let mut position = Position::new(0, 0);
    let mut grid: HashMap<Position, u8> = HashMap::new();

    grid.insert(position, 1);

    loop {
        match computer.run_non_blocking() {
            State::AwaitingInput => {
                let &color = grid.get(&position).unwrap_or(&0);
                computer.enqueue_input(BigInt::from(color));
            }
            State::Outputed(color_num) => {
                let color = color_num.to_u8().unwrap();

                grid.insert(position, color);

                if let State::Outputed(turn_num) = computer.run_non_blocking() {
                    direction = match turn_num.to_u8().unwrap() {
                        0 => direction.turn_left(),
                        1 => direction.turn_right(),
                        _ => panic!(),
                    };
                } else {
                    panic!();
                }

                position = position.advance(direction);
            }
            State::Halted => break,
        }
    }

    grid
}

fn main() -> io::Result<()> {
    let program = Memory::load_line(File::open("input")?)?;
    let mut computer = Computer::new(program);

    let grid = run_paint_job(&mut computer);

    for y in -10..10 {
        for x in (-80..80).rev() {
            if let Some(1) = grid.get(&Position::new(x, y)) {
                print!("■");
            } else {
                print!(" ");
            }
        }

        println!(" ");
    }

    Ok(())
}
