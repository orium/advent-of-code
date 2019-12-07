use itertools::Itertools;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub enum AddressingMode {
    Direct,
    Immediate,
}

impl From<u32> for AddressingMode {
    fn from(v: u32) -> AddressingMode {
        match v {
            0 => AddressingMode::Direct,
            1 => AddressingMode::Immediate,
            _ => panic!("invalid addressing mode"),
        }
    }
}

#[derive(Clone)]
pub struct Computer {
    memory: Vec<i32>,
    program_counter: usize,
}

impl Computer {
    pub fn new(memory: Vec<i32>) -> Computer {
        Computer { memory, program_counter: 0 }
    }

    pub fn load_program<R: Read>(reader: R) -> io::Result<Computer> {
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        reader.read_line(&mut line)?;

        Ok(Computer::new(line.split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect()))
    }

    fn read_argument(&self, argument: i32, mode: AddressingMode) -> i32 {
        match mode {
            AddressingMode::Direct => self.memory[argument as usize],
            AddressingMode::Immediate => argument,
        }
    }

    pub fn run(&mut self, mut input: impl Iterator<Item = i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();

        loop {
            let opcode = self.memory[self.program_counter] % 100;
            let address_mode = |position: u8| -> AddressingMode {
                AddressingMode::from(
                    self.memory[self.program_counter] as u32 / 10u32.pow(position as u32 + 2) % 10,
                )
            };

            match opcode {
                1 | 2 => {
                    let v0 =
                        self.read_argument(self.memory[self.program_counter + 1], address_mode(0));
                    let v1 =
                        self.read_argument(self.memory[self.program_counter + 2], address_mode(1));
                    let store_addr = self.memory[self.program_counter + 3];

                    self.memory[store_addr as usize] = match opcode {
                        1 => v0 + v1,
                        2 => v0 * v1,
                        _ => unreachable!(),
                    };

                    self.program_counter += 4;
                }
                3 => {
                    let v: i32 = input.next().expect("No more input");
                    let addr = self.memory[self.program_counter + 1] as usize;
                    self.memory[addr] = v;
                    self.program_counter += 2;
                }
                4 => {
                    let v =
                        self.read_argument(self.memory[self.program_counter + 1], address_mode(0));
                    output.push(v);
                    self.program_counter += 2;
                }
                5 | 6 => {
                    let jump_on: bool = opcode == 5;
                    let cond =
                        self.read_argument(self.memory[self.program_counter + 1], address_mode(0));
                    let jump_to =
                        self.read_argument(self.memory[self.program_counter + 2], address_mode(1));

                    self.program_counter = match (cond != 0) == jump_on {
                        true => jump_to as usize,
                        false => self.program_counter + 3,
                    };
                }
                7 | 8 => {
                    let v0 =
                        self.read_argument(self.memory[self.program_counter + 1], address_mode(0));
                    let v1 =
                        self.read_argument(self.memory[self.program_counter + 2], address_mode(1));
                    let store_addr = self.memory[self.program_counter + 3] as usize;

                    self.memory[store_addr] = match opcode {
                        7 => v0 < v1,
                        8 => v0 == v1,
                        _ => unreachable!(),
                    } as i32;

                    self.program_counter += 4;
                }
                99 => break,
                opcode => panic!("Invalid opcode {}", opcode),
            }
        }

        output
    }
}

fn compute_amplification(original_computer: &Computer, phases: &[i32]) -> i32 {
    let mut last_output = 0;

    for phase in phases {
        let mut computer = original_computer.clone();
        let input_stream: Vec<i32> = vec![*phase, last_output];
        let output = computer.run(input_stream.iter().map(|v| *v));
        last_output = output[0];
    }

    last_output
}

fn main() -> io::Result<()> {
    const LENGTH: usize = 5;
    let original_computer = Computer::load_program(std::io::stdin())?;
    let mut best_amplification = std::i32::MIN;

    for phases in (0..=4).permutations(LENGTH) {
        let amplification = compute_amplification(&original_computer, &phases);
        best_amplification = best_amplification.max(amplification);
    }

    println!("{}", best_amplification);

    Ok(())
}
