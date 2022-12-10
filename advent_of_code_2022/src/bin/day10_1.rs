use scan_fmt::scan_fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/10");

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        if let Ok(v) = scan_fmt!(s, "addx {}", i64) {
            return Ok(Instruction::AddX(v));
        }

        Err(())
    }
}

#[derive(Copy, Clone, Debug)]
struct Processor {
    x: i64,
    cycle: usize,
}

impl Processor {
    fn new() -> Processor {
        Processor { x: 1, cycle: 0 }
    }

    fn execute(&mut self, instruction: Instruction) {
        self.cycle = self.cycle + instruction.cycles();

        match instruction {
            Instruction::Noop => (),
            Instruction::AddX(v) => self.x = self.x + v,
        }
    }
}

fn main() {
    let mut processor = Processor::new();
    let mut sum: i64 = 0;

    for line in INPUT.lines() {
        let instruction = Instruction::from_str(line).unwrap();

        for cycle in (processor.cycle + 1)..=(processor.cycle + instruction.cycles()) {
            if (cycle + 20) % 40 == 0 {
                sum += cycle as i64 * processor.x;
            }
        }

        processor.execute(instruction);
    }

    println!("{}", sum);
}
