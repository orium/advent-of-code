use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/07");

fn main() {
    // Map from line to list of splitters.
    let splitters: Vec<HashSet<usize>> =
        INPUT.lines().map(|line| line.chars().positions(|c| c == '^').collect()).collect();
    let (start_beam, _) =
        INPUT.lines().next().unwrap().chars().find_position(|c| *c == 'S').unwrap();

    let mut beams: HashSet<usize> = [start_beam].into_iter().collect();
    let mut count = 0;

    for splitters in splitters {
        let new_beams = beams
            .intersection(&splitters)
            .flat_map(|x| [x - 1, x + 1])
            .chain(beams.difference(&splitters).copied())
            .collect();

        count += beams.intersection(&splitters).count();
        beams = new_beams;
    }

    println!("{count}");
}
