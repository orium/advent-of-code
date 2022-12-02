use std::cmp::Ordering;
use std::io;
use std::io::{BufRead, BufReader};

// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
// A for Rock, B for Paper, and C for Scissors.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_letter(s: &str) -> Option<RPS> {
        match s {
            "A" => Some(RPS::Rock),
            "B" => Some(RPS::Paper),
            "C" => Some(RPS::Scissors),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl RoundResult {
    fn from_letter(s: &str) -> Option<RoundResult> {
        match s {
            "X" => Some(RoundResult::Lose),
            "Y" => Some(RoundResult::Draw),
            "Z" => Some(RoundResult::Win),
            _ => None,
        }
    }
}

fn result(other: RPS, mine: RPS) -> RoundResult {
    match mine.partial_cmp(&other) {
        Some(Ordering::Less) => RoundResult::Lose,
        None | Some(Ordering::Equal) => RoundResult::Draw,
        Some(Ordering::Greater) => RoundResult::Win,
    }
}

fn score(other: RPS, mine: RPS) -> usize {
    // The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
    // plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
    let points_match = match result(other, mine) {
        RoundResult::Lose => 0,
        RoundResult::Draw => 3,
        RoundResult::Win => 6,
    };

    points_match + mine.points()
}

fn pick_mine(other: RPS, res: RoundResult) -> RPS {
    [RPS::Rock, RPS::Paper, RPS::Scissors].iter().copied().find(|mine| result(other, *mine) == res).unwrap()
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut total_score: usize = 0;

    for line in reader.lines() {
        let line = line?;
        let (other, res) = line.split_once(" ").unwrap();
        let (other, res) = (RPS::from_letter(other).unwrap(), RoundResult::from_letter(res).unwrap());

        let mine = pick_mine(other, res);

        println!("{:?} {:?} {:?} {}", other, res, mine, score(other, mine));

        total_score += score(other, mine);
    }

    println!("{}", total_score);

    Ok(())
}
