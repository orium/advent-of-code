use std::io;
use std::io::{BufRead, BufReader, Read};

fn load_program<R: Read>(reader: R) -> io::Result<Vec<i32>> {
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line)?;

    Ok(line.split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect())
}

fn execute_bin_op(memory: &mut Vec<i32>, program_counter: usize, op: impl Fn(i32, i32) -> i32) {
    let v0_addr = memory[program_counter + 1];
    let v1_addr = memory[program_counter + 2];
    let store_addr = memory[program_counter + 3];
    let v0 = memory[v0_addr as usize];
    let v1 = memory[v1_addr as usize];

    memory[store_addr as usize] = op(v0, v1);
}

fn run_program(memory: &mut Vec<i32>) {
    let mut program_counter = 0;

    loop {
        match memory[program_counter] {
            1 => {
                execute_bin_op(memory, program_counter, |a, b| a + b);
                program_counter += 4;
            }
            2 => {
                execute_bin_op(memory, program_counter, |a, b| a * b);
                program_counter += 4;
            }
            99 => break,
            _ => panic!(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut program = load_program(io::stdin())?;

    program[1] = 12;
    program[2] = 2;

    run_program(&mut program);

    println!("{}", program[0]);

    Ok(())
}
