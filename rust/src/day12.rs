// use ndarray::Array2;
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
use nom::IResult;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Move(char, isize);

pub fn parse_input<R: Read>(r: R) -> Vec<Move> {
    let reader = BufReader::new(r);
    reader
        .lines()
        .map(|l| parse_move(&l.unwrap()).unwrap().1)
        .collect()
}

fn parse_move(i: &str) -> IResult<&str, Move> {
    use nom::character::complete::*;
    use nom::combinator::map;

    let (i, dir) = anychar(i)?;
    let (i, num) = map(digit1, str::parse::<isize>)(i)?;
    Ok((i, Move(dir, num.unwrap())))
}

pub fn part1(xs: &[Move]) -> usize {
    let mut coord = (0, 0);
    let mut facing = 90;

    for Move(c, n) in xs {
        match c {
            'L' => {
                facing = (facing + 360 - n) % 360;
            }
            'R' => {
                facing = (facing + n) % 360;
            }
            'F' => {
                match facing {
                    0 => {
                        coord = (coord.0, coord.1 + n);
                    } // north
                    90 => {
                        coord = (coord.0 + n, coord.1);
                    } // east
                    180 => {
                        coord = (coord.0, coord.1 - n);
                    } // south
                    270 => {
                        coord = (coord.0 - n, coord.1);
                    } // west
                    x => panic!("unexpected: {}", x),
                };
            }
            'N' => {
                coord = (coord.0, coord.1 + n);
            }
            'E' => {
                coord = (coord.0 + n, coord.1);
            }
            'S' => {
                coord = (coord.0, coord.1 - n);
            }
            'W' => {
                coord = (coord.0 - n, coord.1);
            }
            _ => unreachable!(),
        };
        dbg!(Move(*c, *n), &coord, &facing);
    }

    coord.0.abs() as usize + coord.1.abs() as usize
}
