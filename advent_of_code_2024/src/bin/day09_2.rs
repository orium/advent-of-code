const INPUT: &str = include_str!("../../inputs/09");

#[derive(Debug)]
struct Slice {
    index: usize,
    size: usize,
}

struct Mem {
    m: Vec<Option<u16>>,
    free_list: Vec<Slice>,
    used_list: Vec<Slice>,
}

fn mem(s: &str) -> Mem {
    let iter = s.chars().map(|c| c.to_digit(10).unwrap() as usize).enumerate();
    let mem = iter
        .clone()
        .flat_map(|(i, v)| match i % 2 == 0 {
            true => std::iter::repeat_n(Some((i / 2) as u16), v),
            false => std::iter::repeat_n(None, v),
        })
        .collect();

    let mut free_list = vec![];
    let mut used_list = vec![];
    let mut index = 0;

    for (i, size) in iter {
        let slice = Slice { index, size };

        match i % 2 == 0 {
            true => used_list.push(slice),
            false => free_list.push(slice),
        }

        index += size;
    }

    Mem { m: mem, free_list, used_list }
}

fn compact(mem: &mut Mem) {
    while let Some(used) = mem.used_list.pop() {
        let free = mem
            .free_list
            .iter_mut()
            .take_while(|free| free.index < used.index)
            .find(|free| free.size >= used.size);

        if let Some(free) = free {
            (0..used.size).for_each(|i| mem.m[free.index + i] = mem.m[used.index + i].take());

            free.index += used.size;
            free.size -= used.size;

            // We don't bother removing free slices with size 0.
        }
    }
}

fn checksum(mem: &Mem) -> u64 {
    mem.m.iter().enumerate().map(|(i, v)| (i as u64) * (v.unwrap_or(0) as u64)).sum()
}

fn main() {
    let mut mem: Mem = mem(INPUT.lines().next().unwrap());

    compact(&mut mem);

    let r = checksum(&mem);

    println!("{r}");
}
