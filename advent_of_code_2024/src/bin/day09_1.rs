const INPUT: &str = include_str!("../../inputs/09");

fn expand(s: &str) -> Vec<Option<u16>> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, v)| match i % 2 == 0 {
            true => std::iter::repeat_n(Some((i / 2) as u16), v),
            false => std::iter::repeat_n(None, v),
        })
        .collect()
}

fn compact(mem: &mut [Option<u16>]) {
    let mut i = 0;
    let mut j = mem.len() - 1;

    while i < j {
        match (mem[i], mem[j]) {
            (Some(_), _) => {
                i += 1;
            }
            (_, None) => {
                j -= 1;
            }
            (None, Some(_)) => {
                mem[i] = mem[j].take();
            }
        }
    }
}

fn checksum(mem: &[Option<u16>]) -> u64 {
    mem.iter().enumerate().map(|(i, v)| (i as u64) * (v.unwrap_or(0) as u64)).sum()
}

fn main() {
    let mut mem: Vec<Option<u16>> = expand(INPUT.lines().next().unwrap());

    compact(&mut mem);

    let r = checksum(&mem);

    println!("{r}");
}
