use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/02");

#[derive(Debug)]
struct Rgb {
    red: u64,
    green: u64,
    blue: u64,
}

impl Rgb {
    const fn new(red: u64, green: u64, blue: u64) -> Rgb {
        Rgb { red, green, blue }
    }

    fn empty() -> Rgb {
        Rgb { red: 0, green: 0, blue: 0 }
    }

    fn add(self, r: u64, g: u64, b: u64) -> Rgb {
        Rgb { red: self.red + r, green: self.green + g, blue: self.blue + b }
    }

    fn all_less_or_eq(&self, other: &Rgb) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

fn parse_game(line: &str) -> Vec<Rgb> {
    line.split("; ")
        .map(|hand| {
            hand.split(", ").fold(Rgb::empty(), |hand, s| {
                let (v, color) = scan_fmt!(s, "{} {}", u64, String).unwrap();

                match color.as_str() {
                    "red" => hand.add(v, 0, 0),
                    "green" => hand.add(0, v, 0),
                    "blue" => hand.add(0, 0, v),
                    _ => unreachable!(),
                }
            })
        })
        .collect()
}

const GAME_POSSIBLE: Rgb = Rgb::new(12, 13, 14);

fn main() {
    let v: u64 = INPUT
        .lines()
        .map(|line| {
            let (game_id, game) = scan_fmt!(line, "Game {d}: {/.*$/}", u64, String).unwrap();
            (game_id, parse_game(&game))
        })
        .filter(|(_, hands)| hands.iter().all(|hand| hand.all_less_or_eq(&GAME_POSSIBLE)))
        .map(|(game_id, _)| game_id)
        .sum();

    println!("{}", v);
}
