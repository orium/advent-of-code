use std::io;
use itertools::Itertools;
use std::io::{Read, BufReader, BufRead};

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
    let mut least_zeros: u32 = std::u32::MAX;
    let mut least_zeros_checksum: u32 = 0;

    for layer in image {
        let mut counts: [u32; 3] = [0; 3];

        for v in layer {
            if v < 3 {
                counts[v as usize] += 1;
            }
        }

        if counts[0] < least_zeros {
            least_zeros = counts[0];
            least_zeros_checksum = counts[1] * counts[2];
        }
    }

    println!("{}", least_zeros_checksum);

    Ok(())
}
