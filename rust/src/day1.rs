use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    rc::Rc,
};

pub fn parseday1<P: AsRef<Path>>(p: P) -> Vec<usize> {
    let fd = File::open(p).unwrap();
    let reader = BufReader::new(fd);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect()
}

pub fn day1p1(xs: &[usize]) -> usize {
    let mut seen = HashSet::new();
    for x in xs {
        let other = 2020 - x;
        if seen.get(&other).is_some() {
            return x * other;
        }
        seen.insert(x);
    }
    panic!("no pair found!");
}

pub fn day1p2(xs: &[usize]) -> usize {
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
