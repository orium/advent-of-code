use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};

enum AddressingMode {
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

fn load_program<R: Read>(reader: R) -> io::Result<Vec<i32>> {
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line)?;

    Ok(line.split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect())
}

fn read_argument(argument: i32, memory: &Vec<i32>, mode: AddressingMode) -> i32 {
    match mode {
        AddressingMode::Direct => memory[argument as usize],
        AddressingMode::Immediate => argument,
    }
}

fn run_program<R: Read, W: Write>(
    memory: &mut Vec<i32>,
    stdin: R,
    mut stdout: W,
) -> io::Result<()> {
    let mut stdin = BufReader::new(stdin);
    let mut program_counter = 0;

    loop {
        let opcode = memory[program_counter] % 100;
        let address_mode = |position: u8| -> AddressingMode {
            AddressingMode::from(
                memory[program_counter] as u32 / 10u32.pow(position as u32 + 2) % 10,
            )
        };

        match opcode {
            1 | 2 => {
                let v0 = read_argument(memory[program_counter + 1], &memory, address_mode(0));
                let v1 = read_argument(memory[program_counter + 2], &memory, address_mode(1));
                let store_addr = memory[program_counter + 3];

                memory[store_addr as usize] = match opcode {
                    1 => v0 + v1,
                    2 => v0 * v1,
                    _ => unreachable!(),
                };

                program_counter += 4;
            }
            3 => {
                let mut line = String::new();
                stdin.read_line(&mut line)?;
                let v: i32 = line.trim().parse().unwrap();
                let addr = memory[program_counter + 1] as usize;
                memory[addr] = v;
                program_counter += 2;
            }
            4 => {
                let v = read_argument(memory[program_counter + 1], &memory, address_mode(0));
                stdout.write_fmt(format_args!("{}\n", v))?;
                program_counter += 2;
            }
            5 | 6 => {
                let jump_on: bool = opcode == 5;
                let cond = read_argument(memory[program_counter + 1], &memory, address_mode(0));
                let jump_to = read_argument(memory[program_counter + 2], &memory, address_mode(1));

                program_counter = match (cond != 0) == jump_on {
                    true => jump_to as usize,
                    false => program_counter + 3,
                };
            }
            7 | 8 => {
                let v0 = read_argument(memory[program_counter + 1], &memory, address_mode(0));
                let v1 = read_argument(memory[program_counter + 2], &memory, address_mode(1));
                let store_addr = memory[program_counter + 3] as usize;

                memory[store_addr] = match opcode {
                    7 => v0 < v1,
                    8 => v0 == v1,
                    _ => unreachable!(),
                } as i32;

                program_counter += 4;
            }
            99 => break,
            opcode => panic!("Invalid opcode {}", opcode),
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut program = load_program(File::open("input")?)?;

    run_program(&mut program, io::stdin(), io::stdout())?;

    Ok(())
}
