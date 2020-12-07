use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn parseday3<P: AsRef<Path>>(p: P) -> ndarray::Array2<u8> {
    let fd = File::open(p).unwrap();
    let reader = BufReader::new(fd);
    let mut n = 0;
    let matrix = reader.lines().fold(Vec::new(), |mut acc, line| {
        let line = line.unwrap();
        n = line.len();
        for c in line.chars() {
            let x = match c {
                '.' => 0u8,
                '#' => 1,
                _ => panic!("unexpected character {}", c),
            };
            acc.push(x);
        }
        acc
    });
    let m = matrix.len() / n;
    ndarray::Array2::from_shape_vec((m, n), matrix).unwrap()
}

fn collisions(slope: (usize, usize), matrix: &ndarray::Array2<u8>) -> usize {
    let (modx, mody) = slope;
    let n_columns = matrix.shape()[1];
    let n_rows = matrix.shape()[0];
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    loop {
        x += modx;
        y += mody;
        if y > n_rows - 1 {
            return trees;
        }
        let coord = (y, x % n_columns);
        trees += *matrix.get(coord).unwrap() as usize;
    }
}

pub fn day3p1(matrix: &ndarray::Array2<u8>) -> usize {
    let slope = (3, 1);
    collisions(slope, matrix)
}

pub fn day3p2(matrix: &ndarray::Array2<u8>) -> usize {
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|&s| collisions(s, matrix)).product()
}
