use nom::IResult;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_input<R: Read>(r: R) -> (usize, Vec<Option<usize>>) {
    let i = {
        let mut reader = BufReader::new(r);
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();
        buf
    };
    let (_, parsed) = parseparse(&i).unwrap();

    parsed
}

fn parseparse(i: &str) -> IResult<&str, (usize, Vec<Option<usize>>)> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::*;
    use nom::combinator::{map, map_opt};
    use nom::multi::separated_list1;

    let (i, departure) = map_opt(digit1, |x: &str| x.parse::<usize>().ok())(i)?;
    let (i, _) = newline(i)?;
    let (i, busses) = separated_list1(
        tag(","),
        alt((
            map(digit1, |x: &str| x.parse::<usize>().ok()),
            map(char('x'), |_| None),
        )),
    )(i)?;

    Ok((i, (departure, busses)))
}

pub fn part1(departure: usize, busses: &[Option<usize>]) -> usize {
    let (bus, closest) = busses
        .iter()
        .filter_map(|x| {
            x.map(|d| {
                let next_mod = if departure % d == 0 {
                    departure / d
                } else {
                    (departure / d) + 1
                };
                (d, next_mod * d)
            })
        })
        .min_by_key(|(_, x)| *x)
        .unwrap();
    bus * (closest - departure)
}

pub fn part2(busses: &[Option<usize>]) -> usize {
    // let mut i = 0;
    // let first_bus = busses[0].unwrap();
    // loop {
    //     let ts = i * first_bus;
    //     if busses
    //         .iter()
    //         .enumerate()
    //         .all(|(offset, x)| x.map(|d| (ts + offset) % d == 0).unwrap_or(true))
    //     {
    //         return ts;
    //     }
    //     i += 1;
    // }

    // chinese remainder seive that I just about understand
    let mut sol = 0;
    let mut lcd = 1;
    for (offset, bus) in busses
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(o, x)| (o, x.unwrap()))
    {
        while (sol + offset) % bus != 0 {
            sol += lcd;
        }
        lcd *= bus;
    }
    sol
}
