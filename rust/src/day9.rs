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
    for (i, &x) in xs.iter().skip(25).enumerate() {
        if !has_pair(&xs[i..25 + i], x) {
            return x;
        }
    }
    panic!("no pair found");
}

fn has_pair(haystack: &[usize], target: usize) -> bool {
    let mut seen = HashSet::with_capacity(25);
    for x in haystack.iter().filter(|x| x < &&target) {
        let other = target - x;
        if seen.contains(&other) {
            return true;
        }
        seen.insert(x);
    }
    false
}

pub fn part2(xs: &[usize]) -> usize {
    let mut min_i = 0;
    let mut max_i = 0;
    let target = part1(xs);

    while max_i < xs.len() {
        assert!(min_i <= max_i);
        let sum: usize = xs[min_i..max_i].iter().sum();
        if sum == target {
            let (min, max) = xs[min_i..max_i]
                .iter()
                .fold((usize::MAX, usize::MIN), |(min, max), &x| {
                    (x.min(min), x.max(max))
                });
            return min + max;
        }
        if sum < target {
            max_i += 1;
        } else {
            min_i += 1;
        }
    }
    panic!("could not find contiguous block");
}
