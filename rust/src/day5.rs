use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn parseday5<P: AsRef<Path>>(p: P) -> Vec<String> {
    let fd = File::open(p).unwrap();
    let reader = BufReader::new(fd);
    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn day5p1(xs: &[String]) -> usize {
    xs.iter().map(Seat::from).map(|s| s.num()).max().unwrap()
}

struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn num(&self) -> usize {
        self.row * 8 + self.col
    }

    fn from<S: AsRef<str>>(pass: S) -> Self {
        find_seat(pass.as_ref())
    }
}

fn find_seat(pass: &str) -> Seat {
    const MAXROW: usize = 127;
    const COLMAX: usize = 7;

    let pass_chars: Vec<char> = pass.chars().collect();

    let mut row_ceil = MAXROW;
    let mut row_floor = 0;
    for i in 0..7 {
        match pass_chars[i] {
            'F' => {
                // lower
                row_ceil -= (row_ceil + 1 - row_floor) / 2;
            }
            'B' => {
                // upper
                row_floor += (row_ceil + 1 - row_floor) / 2;
            }
            _ => panic!("invalid row input"),
        }
    }
    assert_eq!(row_ceil, row_floor);
    let row = row_floor;

    let mut col_ceil = COLMAX;
    let mut col_floor = 0;
    for i in 7..10 {
        match pass_chars[i] {
            'L' => {
                // lower
                col_ceil -= (col_ceil + 1 - col_floor) / 2;
            }
            'R' => {
                // upper
                col_floor += (col_ceil + 1 - col_floor) / 2;
            }
            _ => panic!("invalid col input"),
        }
    }
    assert_eq!(col_ceil, col_floor);
    let col = col_floor;

    Seat { row, col }
}

pub fn day5p2(xs: &[String]) -> usize {
    let seats: HashSet<usize> = xs.iter().map(Seat::from).map(|x| x.num()).collect();
    let (min, max) = seats
        .iter()
        .fold((std::usize::MAX, std::usize::MIN), |(min, max), &x| {
            (std::cmp::min(min, x), std::cmp::max(max, x))
        });
    let candidates: Vec<usize> = (min..max).filter(|x| !seats.contains(x)).collect();
    *candidates
        .iter()
        .find(|&x| seats.contains(&(x + 1)) && seats.contains(&(x - 1)))
        .unwrap()
}
