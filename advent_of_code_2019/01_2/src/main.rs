use std::io;
use std::io::{BufRead, BufReader, Read};

fn fuel_for_module(mass: u32) -> u32 {
    match mass {
        0 => 0,
        _ => {
            let fuel = (mass as i64 / 3 - 2).max(0) as u32;
            fuel + fuel_for_module(fuel)
        }
    }
}

fn read_and_compute<R: Read>(reader: R) -> io::Result<u32> {
    let reader = BufReader::new(reader);
    let mut total_fuel = 0;

    for line in reader.lines() {
        let mass: u32 = line?.parse().unwrap();
        total_fuel += fuel_for_module(mass);
    }

    Ok(total_fuel)
}

fn main() -> io::Result<()> {
    let total_fuel = read_and_compute(io::stdin())?;

    println!("{}", total_fuel);

    Ok(())
}
