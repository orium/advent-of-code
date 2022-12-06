const INPUT: &str = include_str!("../input");

fn main() {
    let mut lines = INPUT.lines();
    let mut stacks: Vec<Vec<char>> = std::iter::repeat(Vec::new()).take(32).collect();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut iter = line.chars();
        let mut i = 0;

        // Example: [N] [C]
        while let Some(_) = iter.next() {
            let c = iter.next().unwrap();

            if c != ' ' {
                stacks[i].push(c);
            }

            // skip the `] `.
            iter.next();
            iter.next();

            i += 1;
        }
    }

    for stack in stacks.iter_mut() {
        stack.pop();
        stack.reverse();
    }

    while let Some(line) = lines.next() {
        let mut p = line.split(' ');
        p.next();
        let count = p.next().unwrap().parse::<usize>().unwrap();
        p.next();
        let origin = p.next().unwrap().parse::<usize>().unwrap() - 1;
        p.next();
        let dest = p.next().unwrap().parse::<usize>().unwrap() - 1;
        let origin_len = stacks[origin].len();

        for i in 0..count {
            let c = stacks[origin][origin_len - count + i];
            stacks[dest].push(c);
            println!("taking {} from {} to {}", c, origin, dest);
        }

        for _ in 0..count {
            stacks[origin].pop();
        }
    }

    for stack in stacks {
        if let Some(c) = stack.last() {
            print!("{}", c);
        }
    }

    println!();
}
