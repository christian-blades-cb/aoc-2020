use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn parseday6<P: AsRef<Path>>(p: P) -> Vec<Vec<String>> {
    let mut fd = File::open(p).unwrap();
    let mut buf = String::new();
    fd.read_to_string(&mut buf).unwrap();

    buf.split("\n\n")
        .map(|group| group.lines().map(String::from).collect())
        .collect()
}

pub fn day6p1(xs: &[Vec<String>]) -> usize {
    xs.iter()
        .map(|group| {
            group
                .iter()
                .fold(HashSet::new(), |mut acc, response| {
                    for c in response.chars() {
                        acc.insert(c);
                    }
                    acc
                })
                .len()
        })
        .sum()
}

pub fn day6p2(xs: &[Vec<String>]) -> usize {
    xs.iter()
        .map(|group| {
            let agreed: HashSet<char> = group[0].chars().collect();
            group
                .iter()
                .fold(agreed, |acc, response| {
                    let set: HashSet<char> = response.chars().collect();
                    acc.intersection(&set).cloned().collect()
                })
                .len()
        })
        .sum()
}
