use itertools::Itertools;
use std::io;
use std::io::{BufRead, BufReader, Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn read_image<R: Read>(reader: R) -> io::Result<Vec<Vec<u32>>> {
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    let mut image = Vec::new();

    reader.read_line(&mut line)?;

    for layer in line.trim().chars().chunks(WIDTH * HEIGHT).into_iter() {
        image.push(layer.map(|v| v.to_digit(10).unwrap()).collect_vec());
    }

    Ok(image)
}

fn main() -> io::Result<()> {
    let image = read_image(io::stdin())?;
    let mut rendered = image.last().unwrap().clone();

    for layer in image.iter().rev().skip(1) {
        for (&v, r) in layer.iter().zip(rendered.iter_mut()) {
            if v != 2 {
                *r = v;
            }
        }
    }

    for row in rendered.chunks(WIDTH) {
        for &v in row {
            if v == 1 {
                print!("█");
            } else {
                print!(" ")
            }
        }
        println!();
    }

    Ok(())
}
