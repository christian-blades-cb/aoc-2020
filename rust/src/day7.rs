use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

type Bag = String;

#[derive(Debug)]
pub struct BagRules {
    container: Bag,
    holds: Option<Vec<(usize, Bag)>>,
}

pub fn parseday7<P: AsRef<Path>>(p: P) -> Vec<BagRules> {
    let fd = File::open(p).unwrap();
    let reader = BufReader::new(fd);
    // let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    // dbg!(&lines[0]);
    // vec![parse_rule(&lines[0]).unwrap().1]
    reader
        .lines()
        .map(|l| parse_rule(&l.unwrap()).unwrap().1)
        .collect()
}

fn parse_rule(i: &str) -> IResult<&str, BagRules> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::combinator::map;
    use nom::multi::separated_list1;

    let (i, container) = bag(i)?;
    let (i, _) = tag(" contain ")(i)?;
    let (i, holds) = alt((
        map(separated_list1(tag(", "), bags), Some),
        map(tag("no other bags"), |_| None),
    ))(i)?;

    Ok((i, BagRules { container, holds }))
}

fn bags(i: &str) -> IResult<&str, (usize, Bag)> {
    use nom::character::complete::*;

    let (i, num) = digit1(i)?;
    let (i, _) = space1(i)?;
    let (i, b) = bag(i)?;
    Ok((i, (num.parse::<usize>().unwrap(), b)))
}

fn bag(i: &str) -> IResult<&str, Bag> {
    use nom::bytes::complete::tag;
    use nom::character::complete::*;
    use nom::combinator::opt;

    let (i, pattern) = alpha1(i)?;
    let (i, _) = space1(i)?;
    let (i, color) = alpha1(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = tag("bag")(i)?;
    let (i, _) = opt(char('s'))(i)?;

    Ok((i, format!("{} {}", pattern, color)))
}

pub fn day7p1(xs: &[BagRules]) -> usize {
    let heldby = xs.iter().filter(|x| x.holds.is_some()).fold(
        HashMap::<Bag, Vec<Bag>>::new(),
        |mut acc, rule| {
            let BagRules { container, holds } = rule;
            for (_, held) in holds.as_ref().unwrap().iter() {
                acc.entry(Clone::clone(held))
                    .and_modify(|x| x.push(Clone::clone(container)))
                    .or_insert_with(|| vec![Clone::clone(container)]);
            }
            acc
        },
    );
    let mut visited = HashSet::new();
    let mut stack: Vec<Bag> = heldby.get("shiny gold").unwrap().iter().cloned().collect();

    while let Some(x) = stack.pop() {
        if visited.insert(Clone::clone(&x)) {
            if let Some(containers) = heldby.get(&x) {
                for c in containers.iter().cloned() {
                    stack.push(c);
                }
            }
        }
    }
    visited.len()
}

pub fn day7p2(xs: &[BagRules]) -> usize {
    use topological_sort::TopologicalSort;
    let (rules, mut topology) = xs.iter().fold(
        (HashMap::new(), TopologicalSort::<&str>::new()),
        |(mut rules, mut topology), BagRules { container, holds }| {
            rules.insert(container, holds);
            if let Some(bags) = holds {
                for (_n, prec) in bags {
                    topology.add_dependency(prec.as_str(), container.as_str());
                }
            }
            (rules, topology)
        },
    );

    let mut resolved = HashMap::<&str, usize>::new();
    while let Some(bag) = topology.pop() {
        let sz = 1 + rules
            .get(&bag.to_string())
            .unwrap()
            .as_ref()
            .map(|holds| {
                holds
                    .iter()
                    .map(|(n, x)| resolved.get(x.as_str()).unwrap() * n)
                    .sum()
            })
            .unwrap_or(0);
        resolved.insert(bag, sz);
        if bag == "shiny gold" {
            // number of bags INSIDE the gold bag, not number of bags total
            return *resolved.get("shiny gold").unwrap() - 1;
        }
    }
    assert!(topology.is_empty());

    panic!("expected shiny gold bag in the topology");
}
