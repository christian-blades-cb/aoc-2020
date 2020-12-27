use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_input<R: Read>(r: R) -> HashSet<Coord> {
    let mut grid = HashSet::new();

    let reader = BufReader::new(r);

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    grid.insert(Coord(x as isize, y as isize, 0));
                }
                _ => unreachable!(),
            }
        }
    }

    grid
}

pub fn part1(initial_state: &HashSet<Coord>) -> usize {
    let final_state = (0..6).fold(Clone::clone(initial_state), |acc, _| cycle(&acc));
    final_state.len()
}

fn cycle(current: &HashSet<Coord>) -> HashSet<Coord> {
    let mut next = HashSet::new();

    // currently_on
    for c in current.iter() {
        let active_neighbors = c
            .neighbors()
            .iter()
            .filter(|c1| current.contains(c1))
            .take(4)
            .count();
        if active_neighbors == 2 || active_neighbors == 3 {
            next.insert(Clone::clone(c));
        }
    }

    // currently_off
    // only check coords near currently-on coords since they need 3 neighbors to turn on
    let candidates: HashMap<Coord, usize> = current.iter().fold(HashMap::new(), |mut acc, c| {
        for a in c.neighbors() {
            acc.entry(a).and_modify(|e| *e += 1).or_insert(1);
        }
        acc
    });
    for (c, _) in candidates.iter().filter(|(_c, &x)| x == 3) {
        next.insert(Clone::clone(c));
    }
    next
}

pub fn part2(initial_state: &HashSet<Coord>) -> usize {
    let state: HashSet<Coord4d> = initial_state.iter().map(Coord4d::from).collect();
    let final_state = (0..6).fold(state, |acc, _| cycle4d(&acc));
    final_state.len()
}

fn cycle4d(current: &HashSet<Coord4d>) -> HashSet<Coord4d> {
    let mut next = HashSet::new();

    // currently_on
    for c in current.iter() {
        let active_neighbors = c
            .neighbors()
            .iter()
            .filter(|c1| current.contains(c1))
            .take(4)
            .count();
        if active_neighbors == 2 || active_neighbors == 3 {
            next.insert(Clone::clone(c));
        }
    }

    // currently_off
    // only check coords near currently-on coords since they need 3 neighbors to turn on
    let candidates: HashMap<Coord4d, usize> = current.iter().fold(HashMap::new(), |mut acc, c| {
        for a in c.neighbors() {
            acc.entry(a).and_modify(|e| *e += 1).or_insert(1);
        }
        acc
    });
    for (c, _) in candidates.iter().filter(|(_c, &x)| x == 3) {
        next.insert(Clone::clone(c));
    }
    next
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Coord4d(isize, isize, isize, isize);

impl Coord4d {
    fn neighbors(&self) -> Vec<Self> {
        let Self(x, y, z, w) = self;
        let (x, y, z, w) = (*x, *y, *z, *w);
        vec![
            // same zplane
            Coord4d(x - 1, y, z, w),
            Coord4d(x + 1, y, z, w),
            Coord4d(x - 1, y - 1, z, w),
            Coord4d(x, y - 1, z, w),
            Coord4d(x + 1, y - 1, z, w),
            Coord4d(x - 1, y + 1, z, w),
            Coord4d(x, y + 1, z, w),
            Coord4d(x + 1, y + 1, z, w),
            // upper zplane
            Coord4d(x - 1, y - 1, z - 1, w),
            Coord4d(x, y - 1, z - 1, w),
            Coord4d(x + 1, y - 1, z - 1, w),
            Coord4d(x - 1, y, z - 1, w),
            Coord4d(x, y, z - 1, w),
            Coord4d(x + 1, y, z - 1, w),
            Coord4d(x - 1, y + 1, z - 1, w),
            Coord4d(x, y + 1, z - 1, w),
            Coord4d(x + 1, y + 1, z - 1, w),
            // lower zplane
            Coord4d(x - 1, y - 1, z + 1, w),
            Coord4d(x, y - 1, z + 1, w),
            Coord4d(x + 1, y - 1, z + 1, w),
            Coord4d(x - 1, y, z + 1, w),
            Coord4d(x, y, z + 1, w),
            Coord4d(x + 1, y, z + 1, w),
            Coord4d(x - 1, y + 1, z + 1, w),
            Coord4d(x, y + 1, z + 1, w),
            Coord4d(x + 1, y + 1, z + 1, w),
            // ** upper wplane
            // same zplane
            Coord4d(x - 1, y, z, w - 1),
            Coord4d(x, y, z, w - 1),
            Coord4d(x + 1, y, z, w - 1),
            Coord4d(x - 1, y - 1, z, w - 1),
            Coord4d(x, y - 1, z, w - 1),
            Coord4d(x + 1, y - 1, z, w - 1),
            Coord4d(x - 1, y + 1, z, w - 1),
            Coord4d(x, y + 1, z, w - 1),
            Coord4d(x + 1, y + 1, z, w - 1),
            // upper zplane
            Coord4d(x - 1, y - 1, z - 1, w - 1),
            Coord4d(x, y - 1, z - 1, w - 1),
            Coord4d(x + 1, y - 1, z - 1, w - 1),
            Coord4d(x - 1, y, z - 1, w - 1),
            Coord4d(x, y, z - 1, w - 1),
            Coord4d(x + 1, y, z - 1, w - 1),
            Coord4d(x - 1, y + 1, z - 1, w - 1),
            Coord4d(x, y + 1, z - 1, w - 1),
            Coord4d(x + 1, y + 1, z - 1, w - 1),
            // lower zplane
            Coord4d(x - 1, y - 1, z + 1, w - 1),
            Coord4d(x, y - 1, z + 1, w - 1),
            Coord4d(x + 1, y - 1, z + 1, w - 1),
            Coord4d(x - 1, y, z + 1, w - 1),
            Coord4d(x, y, z + 1, w - 1),
            Coord4d(x + 1, y, z + 1, w - 1),
            Coord4d(x - 1, y + 1, z + 1, w - 1),
            Coord4d(x, y + 1, z + 1, w - 1),
            Coord4d(x + 1, y + 1, z + 1, w - 1),
            // ** upper wplane
            // same zplane
            Coord4d(x - 1, y, z, w + 1),
            Coord4d(x, y, z, w + 1),
            Coord4d(x + 1, y, z, w + 1),
            Coord4d(x - 1, y - 1, z, w + 1),
            Coord4d(x, y - 1, z, w + 1),
            Coord4d(x + 1, y - 1, z, w + 1),
            Coord4d(x - 1, y + 1, z, w + 1),
            Coord4d(x, y + 1, z, w + 1),
            Coord4d(x + 1, y + 1, z, w + 1),
            // upper zplane
            Coord4d(x - 1, y - 1, z - 1, w + 1),
            Coord4d(x, y - 1, z - 1, w + 1),
            Coord4d(x + 1, y - 1, z - 1, w + 1),
            Coord4d(x - 1, y, z - 1, w + 1),
            Coord4d(x, y, z - 1, w + 1),
            Coord4d(x + 1, y, z - 1, w + 1),
            Coord4d(x - 1, y + 1, z - 1, w + 1),
            Coord4d(x, y + 1, z - 1, w + 1),
            Coord4d(x + 1, y + 1, z - 1, w + 1),
            // lower zplane
            Coord4d(x - 1, y - 1, z + 1, w + 1),
            Coord4d(x, y - 1, z + 1, w + 1),
            Coord4d(x + 1, y - 1, z + 1, w + 1),
            Coord4d(x - 1, y, z + 1, w + 1),
            Coord4d(x, y, z + 1, w + 1),
            Coord4d(x + 1, y, z + 1, w + 1),
            Coord4d(x - 1, y + 1, z + 1, w + 1),
            Coord4d(x, y + 1, z + 1, w + 1),
            Coord4d(x + 1, y + 1, z + 1, w + 1),
        ]
    }
}

impl From<&Coord> for Coord4d {
    fn from(c: &Coord) -> Self {
        let Coord(x, y, z) = c;

        Coord4d(*x, *y, *z, 0)
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub struct Coord(isize, isize, isize);

impl Coord {
    fn neighbors(&self) -> Vec<Self> {
        let Self(x, y, z) = self;
        let (x, y, z) = (*x, *y, *z);
        vec![
            // same zplane
            Coord(x - 1, y, z),
            Coord(x + 1, y, z),
            Coord(x - 1, y - 1, z),
            Coord(x, y - 1, z),
            Coord(x + 1, y - 1, z),
            Coord(x - 1, y + 1, z),
            Coord(x, y + 1, z),
            Coord(x + 1, y + 1, z),
            // upper zplane
            Coord(x - 1, y - 1, z - 1),
            Coord(x, y - 1, z - 1),
            Coord(x + 1, y - 1, z - 1),
            Coord(x - 1, y, z - 1),
            Coord(x, y, z - 1),
            Coord(x + 1, y, z - 1),
            Coord(x - 1, y + 1, z - 1),
            Coord(x, y + 1, z - 1),
            Coord(x + 1, y + 1, z - 1),
            // lower zplane
            Coord(x - 1, y - 1, z + 1),
            Coord(x, y - 1, z + 1),
            Coord(x + 1, y - 1, z + 1),
            Coord(x - 1, y, z + 1),
            Coord(x, y, z + 1),
            Coord(x + 1, y, z + 1),
            Coord(x - 1, y + 1, z + 1),
            Coord(x, y + 1, z + 1),
            Coord(x + 1, y + 1, z + 1),
        ]
    }
}
