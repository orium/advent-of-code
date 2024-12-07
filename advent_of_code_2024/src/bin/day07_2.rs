use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/07");

fn input() -> impl Iterator<Item = (i64, Vec<i64>)> {
    INPUT.lines().filter_map(|l| l.split_once(':')).map(|(r, v)| {
        (r.parse().unwrap(), v.trim().split(' ').map(|v| v.parse().unwrap()).collect_vec())
    })
}

fn cartesian_product_seq<T: Clone>(
    iter: impl Iterator<Item = T> + Clone,
    length: usize,
) -> impl Iterator<Item = Vec<T>> {
    std::iter::repeat_n(iter, length).multi_cartesian_product()
}

fn main() {
    let sum = |a: i64, b: i64| a + b;
    let mul = |a: i64, b: i64| a * b;
    let concat = |a: i64, b: i64| a * 10_i64.pow(b.ilog10() + 1) + b;
    let ops = [sum, mul, concat];

    let calc = |numbers: &[i64], operations: Vec<usize>| {
        numbers.iter().zip([0].into_iter().chain(operations)).fold(0, |a, (&v, op)| ops[op](a, v))
    };

    let r: i64 = input()
        .filter(|(result, numbers)| {
            cartesian_product_seq(0..ops.len(), numbers.len() - 1)
                .any(|operations| calc(numbers, operations) == *result)
        })
        .map(|(result, _)| result)
        .sum();

    println!("{r}");
}
