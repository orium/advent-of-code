use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/07");

fn main() {
    // Map from line to list of splitters.
    let splitters: Vec<HashSet<usize>> =
        INPUT.lines().map(|line| line.chars().positions(|c| c == '^').collect()).collect();
    let (start_beam, _) =
        INPUT.lines().next().unwrap().chars().find_position(|c| *c == 'S').unwrap();

    let mut beams: HashSet<usize> = [start_beam].into_iter().collect();
    let mut paths: HashMap<usize, usize> = [(start_beam, 1)].into_iter().collect();

    for splitters in splitters {
        let new_beams = beams
            .intersection(&splitters)
            .flat_map(|x| [x - 1, x + 1])
            .chain(beams.difference(&splitters).copied())
            .collect();
        let new_paths = beams
            .intersection(&splitters)
            .flat_map(|x| [(x - 1, paths[x]), (x + 1, paths[x])])
            .chain(beams.difference(&splitters).map(|x| (*x, paths[x])))
            .into_grouping_map()
            .sum();

        beams = new_beams;
        paths = new_paths;
    }

    let r: usize = paths.values().copied().sum();

    println!("{r}");
}
