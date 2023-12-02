use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/02");

#[derive(Debug)]
struct Rgb {
    red: u64,
    green: u64,
    blue: u64,
}

impl Rgb {
    fn empty() -> Rgb {
        Rgb { red: 0, green: 0, blue: 0 }
    }

    fn add(self, r: u64, g: u64, b: u64) -> Rgb {
        Rgb { red: self.red + r, green: self.green + g, blue: self.blue + b }
    }

    fn max(self, other: Rgb) -> Rgb {
        Rgb {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn pow(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

fn parse_game(line: &str) -> impl Iterator<Item = Rgb> + '_ {
    line.split("; ").map(|hand| {
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
}

fn main() {
    let v: u64 = INPUT
        .lines()
        .map(|line| {
            let game = scan_fmt!(line, "Game {*d}: {/.*$/}", String).unwrap();
            parse_game(&game).reduce(Rgb::max).unwrap().pow()
        })
        .sum();

    println!("{}", v);
}
