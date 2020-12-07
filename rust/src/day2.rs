use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct PasswordRule {
    letter: char,
    min: usize,
    max: usize,
}

pub fn parseday2<P: AsRef<Path>>(p: P) -> Vec<(PasswordRule, String)> {
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

pub fn day2p1(xs: &[(PasswordRule, String)]) -> usize {
    xs.iter()
        .filter(|(rule, pass)| {
            let PasswordRule { letter, min, max } = rule;
            let count = pass.chars().filter(|c| c == letter).count();
            count >= *min && count <= *max
        })
        .count()
}

pub fn day2p2(xs: &[(PasswordRule, String)]) -> usize {
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
