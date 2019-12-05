use itertools::Itertools;

const FROM: u32 = 172851;
const TO: u32 = 675869;

fn explode(mut n: u32) -> [u8; 6] {
    let mut r: [u8; 6] = [0; 6];
    let mut p = 6;

    while n > 0 {
        p -= 1;
        r[p] = (n % 10) as u8;
        n /= 10;
    }

    r
}

fn check_conditions(n: u32) -> bool {
    let exploded = explode(n);

    let ascending = exploded.iter().tuple_windows().all(|(n, m)| n <= m);
    let exactly_two =
        exploded.iter().group_by(|&&v| v).into_iter().map(|(_, g)| g.count()).any(|c| c == 2);

    ascending && exactly_two
}

fn main() {
    let mut count: u32 = 0;

    for password in FROM..=TO {
        count += check_conditions(password) as u32;
    }

    println!("{}", count);
}
