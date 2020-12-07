use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Default)]
pub struct Record {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

pub fn parseday4<P: AsRef<Path>>(p: P) -> Vec<Record> {
    let mut fd = File::open(p).unwrap();
    let mut buf = String::new();
    fd.read_to_string(&mut buf).unwrap();

    let mut acc = Vec::new();
    for record in buf.split("\n\n") {
        let mut passport = Record::default();
        for item in record.split_whitespace() {
            let mut pair = item.splitn(2, ':').map(String::from);
            match pair.next().unwrap().as_str() {
                "byr" => passport.byr = Some(pair.next().unwrap().into()),
                "iyr" => passport.iyr = Some(pair.next().unwrap().into()),
                "eyr" => passport.eyr = Some(pair.next().unwrap().into()),
                "hgt" => passport.hgt = Some(pair.next().unwrap().into()),
                "hcl" => passport.hcl = Some(pair.next().unwrap().into()),
                "ecl" => passport.ecl = Some(pair.next().unwrap().into()),
                "pid" => passport.pid = Some(pair.next().unwrap().into()),
                "cid" => passport.cid = Some(pair.next().unwrap().into()),
                _ => {}
            }
        }
        acc.push(passport);
    }
    acc
}

pub fn day4p1(xs: &[Record]) -> usize {
    xs.iter()
        .filter(|x| {
            x.byr.is_some()
                && x.iyr.is_some()
                && x.eyr.is_some()
                && x.hgt.is_some()
                && x.hcl.is_some()
                && x.ecl.is_some()
                && x.pid.is_some()
        })
        .count()
}

pub fn day4p2(xs: &[Record]) -> usize {
    let hex_re = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_re = regex::Regex::new(r"^\d{9}$").unwrap();
    xs.iter()
        .filter(|record| {
            let byr = {
                if let Some(byr) = &record.byr {
                    let byr = byr.parse::<usize>().unwrap_or(0);
                    byr >= 1920 && byr <= 2002
                } else {
                    false
                }
            };

            let iyr = {
                record.iyr.as_ref().map(|iyr| {
                    let iyr = iyr.parse::<usize>().unwrap_or(0);
                    iyr >= 2010 && iyr <= 2020
                })
            }
            .unwrap_or(false);

            let eyr = record
                .eyr
                .as_ref()
                .map(|eyr| {
                    let eyr = eyr.parse::<usize>().unwrap_or(0);
                    eyr >= 2020 && eyr <= 2030
                })
                .unwrap_or(false);

            let hgt = {
                record.hgt.as_ref().map(|hgt| {
                    let n = hgt.len();
                    match (hgt.ends_with("in"), hgt.ends_with("cm")) {
                        (true, false) => hgt[0..n - 2]
                            .parse::<usize>()
                            .map(|v| v >= 59 && v <= 76)
                            .unwrap_or(false),
                        (false, true) => hgt[0..n - 2]
                            .parse::<usize>()
                            .map(|v| v >= 150 && v <= 193)
                            .unwrap_or(false),
                        _ => false,
                    }
                })
            }
            .unwrap_or(false);

            let hcl = record
                .hcl
                .as_ref()
                .map(|hcl| hex_re.is_match(hcl))
                .unwrap_or(false);

            let ecl = record
                .ecl
                .as_ref()
                .map(|ecl| match ecl.as_str() {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false,
                })
                .unwrap_or(false);

            let pid = record
                .pid
                .as_ref()
                .map(|pid| pid_re.is_match(pid))
                .unwrap_or(false);

            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count()
}
