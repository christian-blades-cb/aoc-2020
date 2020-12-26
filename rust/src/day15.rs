use std::collections::HashMap;
use std::io::prelude::*;

pub fn parse_input<R: Read>(mut r: R) -> Vec<usize> {
    let mut buf = String::new();
    r.read_to_string(&mut buf).unwrap();
    buf.split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn part1(starters: &[usize]) -> usize {
    play(starters, 2020)
}

#[allow(dead_code)]
pub fn part2(starters: &[usize]) -> usize {
    play(starters, 30_000_000)
}

fn play(starters: &[usize], turns: usize) -> usize {
    assert!(turns > starters.len());

    let mut memory: HashMap<usize, usize> = HashMap::new();
    for (turn, x) in starters.iter().enumerate().map(|(i, x)| (i + 1, x)) {
        memory.insert(*x, turn);
    }

    let mut previous = *starters.last().unwrap();
    for turn in starters.len() + 1..=turns {
        let spoken = if let Some(last_spoken_turn) = memory.get(&previous) {
            turn - 1 - last_spoken_turn // diff between last time spoken and the previous time spoken (last time spoke is always previous turn)
        } else {
            0 // first time spoken
        };
        memory.insert(previous, turn - 1);
        previous = spoken;
    }

    previous
}
