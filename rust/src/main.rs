use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let day1input = parseday1("day1.input");
    println!("day1p1: {}", day1p1(&day1input));
    println!("day1p2: {}", day1p2(&day1input));
    let day2input = parseday2("day2.input");
    println!("day2p1: {}", day2p1(&day2input));
    println!("day2p2: {}", day2p2(&day2input));
}

fn parseday1<P: AsRef<Path>>(p: P) -> Vec<usize> {
    let fd = std::fs::File::open(p).unwrap();
    let reader = std::io::BufReader::new(fd);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect()
}

fn day1p1(xs: &[usize]) -> usize {
    let mut seen = std::collections::HashSet::new();
    for x in xs {
        let other = 2020 - x;
        if seen.get(&other).is_some() {
            return x * other;
        }
        seen.insert(x);
    }
    panic!("no pair found!");
}

fn day1p2(xs: &[usize]) -> usize {
    use std::collections::HashSet;
    use std::rc::Rc;

    let cap = xs.len();
    let pool = xs.iter().fold(HashSet::with_capacity(cap), |mut acc, x| {
        acc.insert(x);
        acc
    });
    let pool = Rc::new(pool);
    let outer_pool = pool.clone();
    let inner_pool = pool.clone();

    for x in outer_pool.iter() {
        let remainder = 2020 - **x;
        for lhs in inner_pool
            .iter()
            .filter(|candidate| ***candidate < remainder)
        {
            let other: usize = remainder - **lhs;
            if Rc::clone(&pool).get(&other).is_some() {
                return **x * **lhs * other;
            }
        }
    }

    panic!("no triple found");
}

struct PasswordRule {
    letter: char,
    min: usize,
    max: usize,
}

fn parseday2<P: AsRef<Path>>(p: P) -> Vec<(PasswordRule, String)> {
    let re =
        regex::Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>.+)").unwrap();
    let fd = File::open(p).unwrap();
    let reader = BufReader::new(fd);

    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let caps = re.captures(&line).unwrap();
            let rule = PasswordRule {
                letter: caps["letter"].chars().next().unwrap(),
                min: caps["min"].parse().unwrap(),
                max: caps["max"].parse().unwrap(),
            };
            (rule, caps["password"].into())
        })
        .collect()
}

fn day2p1(xs: &[(PasswordRule, String)]) -> usize {
    xs.iter()
        .filter(|(rule, pass)| {
            let PasswordRule { letter, min, max } = rule;
            let count = pass.chars().filter(|c| c == letter).count();
            count >= *min && count <= *max
        })
        .count()
}

fn day2p2(xs: &[(PasswordRule, String)]) -> usize {
    xs.iter()
        .filter(|(rule, pass)| {
            let PasswordRule {
                letter,
                min: pos1,
                max: pos2,
            } = rule;
            let pass_chars: Vec<char> = pass.chars().collect();
            let pos1match = pass_chars
                .get(pos1 - 1)
                .map(|c| c == letter)
                .unwrap_or(false);
            let pos2match = pass_chars
                .get(pos2 - 1)
                .map(|c| c == letter)
                .unwrap_or(false);
            match (pos1match, pos2match) {
                (true, false) => true,
                (false, true) => true,
                _ => false,
            }
        })
        .count()
}
