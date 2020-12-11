use nom::IResult;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Default)]
struct Machine {
    i: usize,   // instruction pointer
    acc: isize, // accumulator
}

#[derive(Debug, Clone)]
pub struct Instruction {
    instruction: String,
    args: Vec<isize>,
}

pub fn parseday8<R: Read>(r: R) -> Vec<Instruction> {
    let reader = BufReader::new(r);
    reader
        .lines()
        .map(|l| parse_instruction(l.unwrap().as_ref()).unwrap().1)
        .collect()
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    use nom::character::complete::*;
    use nom::combinator::map;
    use nom::multi::separated_list1;

    let (i, instruction) = map(alpha1, String::from)(i)?;
    let (i, _) = space1(i)?;
    let (i, args) = separated_list1(space1, parse_integer)(i)?;

    Ok((i, Instruction { instruction, args }))
}

fn parse_integer(i: &str) -> IResult<&str, isize> {
    use nom::character::complete::*;
    use nom::combinator::map;

    let (i, pos) = one_of("+-")(i)?;
    let (i, num) = map(digit1, str::parse::<isize>)(i)?;
    let n = match pos {
        '+' => num.unwrap(),
        '-' => 0 - num.unwrap(),
        _ => unreachable!(),
    };
    Ok((i, n))
}

pub fn part1(xs: &[Instruction]) -> isize {
    let mut machine = Machine { i: 0, acc: 0 };
    let mut visited = std::collections::HashSet::new();

    loop {
        let x = &xs[machine.i];
        if !visited.insert(machine.i) {
            return machine.acc;
        }
        let Instruction { instruction, args } = x;
        match instruction.as_str() {
            "nop" => {
                machine.i += 1;
            }
            "jmp" => {
                machine.i = (machine.i as isize + args[0]) as usize;
            }
            "acc" => {
                machine.acc += args[0];
                machine.i += 1;
            }
            _ => panic!("unexpected instruction"),
        }
    }
}

pub fn part2(xs: &[Instruction]) -> isize {
    let mutation_indices = xs.iter().enumerate().filter_map(|(i, instruction)| {
        match instruction.instruction.as_str() {
            "nop" | "jmp" => Some(i),
            _ => None,
        }
    });
    let winner = mutation_indices
        .map(|i| {
            let mutation: Vec<Instruction> = xs
                .iter()
                .enumerate()
                .map(|(j, x)| {
                    if j == i {
                        let Instruction { instruction, args } = x;
                        let new_instruction = match instruction.as_str() {
                            "nop" => "jmp".to_string(),
                            "jmp" => "nop".to_string(),
                            _ => panic!("how did we even get here?"),
                        };
                        Instruction {
                            instruction: new_instruction,
                            args: args.to_vec(),
                        }
                    } else {
                        Clone::clone(x)
                    }
                })
                .collect();
            run(&mutation)
        })
        .find(Option::is_some);
    winner.unwrap().unwrap()
}

fn run(xs: &[Instruction]) -> Option<isize> {
    let mut machine = Machine::default();
    let mut visited = std::collections::HashSet::new();

    while let Some(Instruction { instruction, args }) = xs.get(machine.i) {
        if !visited.insert(machine.i) {
            return None;
        }
        match instruction.as_str() {
            "nop" => {
                machine.i += 1;
            }
            "jmp" => {
                machine.i = (machine.i as isize + args[0]) as usize;
            }
            "acc" => {
                machine.acc += args[0];
                machine.i += 1;
            }
            _ => panic!("unexpected instruction"),
        }
    }

    Some(machine.acc)
}
