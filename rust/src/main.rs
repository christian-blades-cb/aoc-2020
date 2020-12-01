use std::io::prelude::*;
use std::path::Path;

fn main() {
    let day1input = parseday1("day1.input");
    println!("day1p1: {}", day1p1(&day1input));
    println!("day1p2: {}", day1p2(&day1input));
}

fn parseday1<P: AsRef<Path>>(p: P) -> Vec<usize> {
    let fd = std::fs::File::open(p).unwrap();
    let reader = std::io::BufReader::new(fd);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect()
}

fn day1p1(xs: &[usize]) -> usize {
    let mut seen = std::collections::HashSet::new();
    for x in xs {
        let other = 2020 - x;
        if seen.get(&other).is_some() {
            return x * other;
        }
        seen.insert(x);
    }
    panic!("no pair found!");
}

fn day1p2(xs: &[usize]) -> usize {
    use std::collections::HashSet;
    use std::rc::Rc;

    let cap = xs.len();
    let pool = xs.iter().fold(HashSet::with_capacity(cap), |mut acc, x| {
        acc.insert(x);
        acc
    });
    let pool = Rc::new(pool);
    let outer_pool = pool.clone();
    let inner_pool = pool.clone();

    for x in outer_pool.iter() {
        let remainder = 2020 - **x;
        for lhs in inner_pool
            .iter()
            .filter(|candidate| ***candidate < remainder)
        {
            let other: usize = remainder - **lhs;
            if Rc::clone(&pool).get(&other).is_some() {
                return **x * **lhs * other;
            }
        }
    }

    panic!("no triple found");
}
