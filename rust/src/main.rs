// use std::collections::HashMap;
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
    let day3input = parseday3("day3.input");
    println!("day3p1: {}", day3p1(&day3input));
    println!("day3p2: {}", day3p2(&day3input));
    let day4input = parseday4("day4.input");
    println!("day4p1: {}", day4p1(&day4input));
    println!("day4p2: {}", day4p2(&day4input));
    let day5input = parseday5("day5.input");
    println!("day5pt1: {}", day5p1(&day5input));
    println!("day5pt2: {}", day5p2(&day5input));
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

fn parseday3<P: AsRef<Path>>(p: P) -> ndarray::Array2<u8> {
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

fn day3p1(matrix: &ndarray::Array2<u8>) -> usize {
    let slope = (3, 1);
    collisions(slope, matrix)
}

fn day3p2(matrix: &ndarray::Array2<u8>) -> usize {
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|&s| collisions(s, matrix)).product()
}

#[derive(Default)]
struct Record {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn parseday4<P: AsRef<Path>>(p: P) -> Vec<Record> {
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

fn day4p1(xs: &[Record]) -> usize {
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

fn day4p2(xs: &[Record]) -> usize {
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

fn parseday5<P: AsRef<Path>>(p: P) -> Vec<String> {
    let fd = File::open(p).unwrap();
    let reader = BufReader::new(fd);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn day5p1(xs: &[String]) -> usize {
    xs.iter()
        .map(|x| find_seat(x))
        .map(|s| s.num())
        .max()
        .unwrap()
}

struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn num(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn find_seat(pass: &str) -> Seat {
    const MAXROW: usize = 127;
    const COLMAX: usize = 7;

    let pass_chars: Vec<char> = pass.chars().collect();

    let mut row_ceil = MAXROW;
    let mut row_floor = 0;
    for i in 0..7 {
        match pass_chars[i] {
            'F' => {
                // lower
                row_ceil -= (row_ceil + 1 - row_floor) / 2;
            }
            'B' => {
                // upper
                row_floor += (row_ceil + 1 - row_floor) / 2;
            }
            _ => panic!("invalid row input"),
        }
    }
    assert_eq!(row_ceil, row_floor);
    let row = row_floor;

    let mut col_ceil = COLMAX;
    let mut col_floor = 0;
    for i in 7..10 {
        match pass_chars[i] {
            'L' => {
                // lower
                col_ceil -= (col_ceil + 1 - col_floor) / 2;
            }
            'R' => {
                // upper
                col_floor += (col_ceil + 1 - col_floor) / 2;
            }
            _ => panic!("invalid col input"),
        }
    }
    assert_eq!(col_ceil, col_floor);
    let col = col_floor;

    Seat { row, col }
}

fn day5p2(xs: &[String]) -> usize {
    use std::collections::HashSet;
    let seats: HashSet<usize> = xs.iter().map(|x| find_seat(x)).map(|x| x.num()).collect();
    let (min, max) = seats
        .iter()
        .fold((std::usize::MAX, std::usize::MIN), |(min, max), &x| {
            (std::cmp::min(min, x), std::cmp::max(max, x))
        });
    dbg!((min, max));
    let candidates: Vec<usize> = (min..max).filter(|x| !seats.contains(x)).collect();
    *candidates
        .iter()
        .find(|&x| seats.contains(&(x + 1)) && seats.contains(&(x - 1)))
        .unwrap()
}
