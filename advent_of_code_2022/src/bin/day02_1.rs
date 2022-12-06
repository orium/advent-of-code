use std::cmp::Ordering;

const INPUT: &str = include_str!("../../inputs/02");

// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
// A for Rock, B for Paper, and C for Scissors.
// X for Rock, Y for Paper, and Z for Scissors.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_letter(s: &str) -> Option<RPS> {
        match s {
            "A" | "X" => Some(RPS::Rock),
            "B" | "Y" => Some(RPS::Paper),
            "C" | "Z" => Some(RPS::Scissors),
            _ => None,
        }
    }

    fn points(self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl PartialOrd<RPS> for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (RPS::Rock, RPS::Scissors) => Some(Ordering::Greater),
            (RPS::Scissors, RPS::Rock) => Some(Ordering::Less),

            (RPS::Scissors, RPS::Paper) => Some(Ordering::Greater),
            (RPS::Paper, RPS::Scissors) => Some(Ordering::Less),

            (RPS::Paper, RPS::Rock) => Some(Ordering::Greater),
            (RPS::Rock, RPS::Paper) => Some(Ordering::Less),

            (_, _) => None,
        }
    }
}

fn score(other: RPS, mine: RPS) -> usize {
    // The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
    // plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    let points_match = match mine.partial_cmp(&other) {
        Some(Ordering::Less) => 0,
        None | Some(Ordering::Equal) => 3,
        Some(Ordering::Greater) => 6,
    };

    points_match + mine.points()
}

fn main() {
    let mut total_score: usize = 0;

    for line in INPUT.lines() {
        let (other, mine) = line.split_once(" ").unwrap();
        let (other, mine) = (RPS::from_letter(other).unwrap(), RPS::from_letter(mine).unwrap());

        println!("{:?} {:?}", other, mine);

        total_score += score(other, mine);
    }

    println!("{}", total_score);
}
