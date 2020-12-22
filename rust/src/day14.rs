use nom::IResult;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum Operation {
    Mem { offset: usize, value: usize },
    Mask(Vec<char>),
}

pub fn parse_input<R: Read>(r: R) -> Vec<Operation> {
    let reader = BufReader::new(r);
    reader
        .lines()
        .map(|l| parse_operation(l.unwrap().as_ref()).unwrap().1)
        .collect()
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    use nom::branch::alt;
    Ok(alt((parse_mem, parse_mask))(i)?)
}

fn parse_mem(i: &str) -> IResult<&str, Operation> {
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::combinator::map_opt;

    let (i, _) = tag("mem[")(i)?;
    let (i, offset) = map_opt(digit1, |s: &str| s.parse::<usize>().ok())(i)?;
    let (i, _) = tag("] = ")(i)?;
    let (i, value) = map_opt(digit1, |s: &str| s.parse::<usize>().ok())(i)?;

    Ok((i, Operation::Mem { offset, value }))
}

fn parse_mask(i: &str) -> IResult<&str, Operation> {
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::multi::count;

    let (i, _) = tag("mask = ")(i)?;
    let (i, mask) = count(one_of("01X"), 36)(i)?;
    Ok((i, Operation::Mask(mask)))
}

pub fn part1(xs: &[Operation]) -> usize {
    let mut set_mask = 0;
    let mut clear_mask = 0;
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for op in xs {
        match op {
            Operation::Mask(m) => {
                let (set, clear) = mask_to_bitmasks(&m);
                set_mask = set;
                clear_mask = clear;
            }
            Operation::Mem { offset, value } => {
                let value = (value | set_mask) & !clear_mask;
                memory.insert(*offset, value);
            }
        }
    }

    memory.values().sum()
}

fn mask_to_bitmasks(m: &[char]) -> (usize, usize) {
    let set_mask = m.iter().fold(0usize, |mut acc, &x| {
        acc <<= 1;
        if x == '1' {
            acc |= 1;
        }
        acc
    });
    let clear_mask = m.iter().fold(0usize, |mut acc, &x| {
        acc <<= 1;
        if x == '0' {
            acc |= 1;
        }
        acc
    });
    (set_mask, clear_mask)
}

pub fn part2(xs: &[Operation]) -> usize {
    let mut set_mask: Vec<char> = "000000000000000000000000000000X1001X".chars().collect();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for op in xs {
        match op {
            Operation::Mask(m) => {
                set_mask = Clone::clone(m);
            }
            Operation::Mem { offset, value } => {
                let addrs = mask_permutations(&set_mask, *offset);
                for a in addrs {
                    memory.insert(a, *value);
                }
            }
        }
    }

    memory.values().sum()
}

fn mask_permutations(mask: &[char], address: usize) -> Vec<usize> {
    (0..36).into_iter().fold(vec![0], |mut acc, i| {
        let bit = (address >> 35 - i) & 0x1;
        match mask[i] {
            '0' => {
                acc.iter_mut().for_each(|base| *base |= bit << 35 - i);
                acc
            }
            '1' => {
                acc.iter_mut().for_each(|base| *base |= 1 << 35 - i);
                acc
            }
            'X' => acc
                .iter()
                .map(|base| vec![base | (1 << 35 - i), base & !(1 << 35 - i)])
                .flatten()
                .collect(),
            _ => unreachable!(),
        }
    })
}
