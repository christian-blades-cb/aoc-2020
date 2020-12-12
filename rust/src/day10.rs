use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_input<R: Read>(r: R) -> Vec<usize> {
    let reader = BufReader::new(r);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect()
}

pub fn part1(xs: &[usize]) -> usize {
    let chain = form_chain(xs);
    let (ones, threes) =
        chain
            .iter()
            .zip(chain.iter().skip(1))
            .fold((0, 0), |(one, three), (n, m)| match m - n {
                1 => (one + 1, three),
                3 => (one, three + 1),
                _ => (one, three),
            });
    ones * threes
}

fn form_chain(xs: &[usize]) -> Vec<usize> {
    let mut pool: HashSet<_> = xs.iter().collect();
    let mut chain: Vec<usize> = Vec::with_capacity(xs.len());
    chain.push(0);
    while !pool.is_empty() {
        let energy = chain.last().cloned().unwrap();
        let next = (energy..=energy + 3)
            .into_iter()
            .filter(|n| pool.contains(n))
            .min()
            .unwrap();
        pool.remove(&next);
        chain.push(next);
    }
    chain.push(chain.last().unwrap() + 3);
    chain
}

pub fn part2(xs: &[usize]) -> usize {
    let chain = form_chain(xs);
    let mut active = false;
    let mut slices = Vec::new();
    let mut run = 0;
    for diff in chain.iter().zip(chain.iter().skip(1)).map(|(m, n)| n - m) {
        match (diff, active) {
            (1, true) => run += 1,
            (_, true) => {
                slices.push(run);
                run = 0;
                active = false;
            }
            (1, false) => {
                active = true;
                run = 1;
            }
            _ => {}
        }
    }
    if active {
        slices.push(run);
    }
    slices
        .iter()
        .map(|&run| {
            let changeable = if run < 2 { 0 } else { run - 1 };
            let invalid_pos = match changeable {
                x if x > 3 => x as usize * 2usize.pow(x - 1),
                3 => 1,
                _ => 0,
            };
            2usize.pow(changeable) - invalid_pos
        })
        .product()
}
