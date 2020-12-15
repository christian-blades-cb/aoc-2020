use ndarray::Array2;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_input<R: Read>(r: R) -> Array2<char> {
    let reader = BufReader::new(r);
    let (n, matrix) = reader.lines().fold((0, Vec::new()), |(_, mut acc), line| {
        let line = line.unwrap();
        let len = line.len();
        for c in line.chars() {
            acc.push(c);
        }
        (len, acc)
    });
    let m = matrix.len() / n;
    Array2::from_shape_vec((m, n), matrix).unwrap()
}

pub fn part1(matrix: &Array2<char>) -> usize {
    coalesce(matrix, transform)
        .iter()
        .filter(|&&x| x == '#')
        .count()
}

fn hash_matrix(matrix: &Array2<char>) -> u64 {
    let mut hasher = DefaultHasher::new();
    (matrix).hash(&mut hasher);
    hasher.finish()
}

fn coalesce<T: Fn(&Array2<char>) -> Array2<char>>(
    matrix: &Array2<char>,
    transformation: T,
) -> Array2<char> {
    let mut matrix = Clone::clone(matrix);
    loop {
        let old_hash = hash_matrix(&matrix);
        matrix = transformation(&matrix);
        let new_hash = hash_matrix(&matrix);
        if new_hash == old_hash {
            break matrix;
        }
    }
}

pub fn part2(matrix: &Array2<char>) -> usize {
    coalesce(matrix, transform2)
        .iter()
        .filter(|&&x| x == '#')
        .count()
}

fn transform2(matrix: &Matrix) -> Matrix {
    const DIRECTIONS: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];

    let transformed: Vec<char> = matrix
        .indexed_iter()
        .map(|(coord, &c)| {
            if c == '.' {
                return '.';
            }

            let count: usize = DIRECTIONS
                .iter()
                .map(|&slope| match seen_on_slope(coord, slope, matrix) {
                    '.' | 'L' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                })
                .sum();

            match (c, count) {
                ('L', 0) => '#',
                ('#', x) if x >= 5 => 'L',
                _ => c,
            }
        })
        .collect();

    Array2::from_shape_vec(matrix.dim(), transformed).unwrap()
}

type Slope = (isize, isize);
type Matrix = Array2<char>;
type Pos = (usize, usize);

fn seen_on_slope(coord: Pos, slope: Slope, matrix: &Matrix) -> char {
    let mut target = coord;
    while target.0 as isize + slope.0 >= 0 && target.1 as isize + slope.1 >= 0 {
        target = (
            (target.0 as isize + slope.0) as usize,
            (target.1 as isize + slope.1) as usize,
        );
        match matrix.get(target) {
            Some('.') => continue,
            Some(c) => return *c,
            None => return '.',
        }
    }
    '.'
}

fn transform(matrix: &Array2<char>) -> Array2<char> {
    const ADJACENTS: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];
    let transformed: Vec<char> = matrix
        .indexed_iter()
        .map(|((x, y), c)| {
            let count: usize = ADJACENTS
                .iter()
                .filter_map(|(mod_x, mod_y)| {
                    if x as isize + mod_x < 0 {
                        return None;
                    }
                    if y as isize + mod_y < 0 {
                        return None;
                    }
                    let coord = ((x as isize + mod_x) as usize, (y as isize + mod_y) as usize);
                    matrix.get(coord).map(|&c| if c == '#' { 1 } else { 0 })
                })
                .sum();
            match (c, count) {
                ('.', _) => '.',
                ('L', 0) => '#',
                ('L', _) => 'L',
                ('#', x) if x >= 4 => 'L',
                (&d, _) => d,
            }
        })
        .collect();
    Array2::from_shape_vec(matrix.dim(), transformed).unwrap()
}
