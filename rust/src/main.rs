use std::fs::File;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let day1input = day1::parseday1("day1.input");
    println!("day1p1: {}", day1::day1p1(&day1input));
    println!("day1p2: {}", day1::day1p2(&day1input));
    let day2input = day2::parseday2("day2.input");
    println!("day2p1: {}", day2::day2p1(&day2input));
    println!("day2p2: {}", day2::day2p2(&day2input));
    let day3input = day3::parseday3("day3.input");
    println!("day3p1: {}", day3::day3p1(&day3input));
    println!("day3p2: {}", day3::day3p2(&day3input));
    let day4input = day4::parseday4("day4.input");
    println!("day4p1: {}", day4::day4p1(&day4input));
    println!("day4p2: {}", day4::day4p2(&day4input));
    let day5input = day5::parseday5("day5.input");
    println!("day5p1: {}", day5::day5p1(&day5input));
    println!("day5p2: {}", day5::day5p2(&day5input));
    let day6input = day6::parseday6("day6.input");
    println!("day6p1: {}", day6::day6p1(&day6input));
    println!("day6p2: {}", day6::day6p2(&day6input));
    let day7input = day7::parseday7("day7.input");
    println!("day7p1: {}", day7::day7p1(&day7input));
    println!("day7p2: {}", day7::day7p2(&day7input));
    let day8input = day8::parseday8(File::open("day8.input").unwrap());
    println!("day8p1: {}", day8::part1(&day8input));
    println!("day8p2: {}", day8::part2(&day8input));
    let day9input = day9::parse_input(File::open("day9.input").unwrap());
    println!("day9p1: {}", day9::part1(&day9input));
    println!("day9p2: {}", day9::part2(&day9input));
    let day10input = day10::parse_input(File::open("day10.input").unwrap());
    println!("day10p1: {}", day10::part1(&day10input));
    println!("day10p2: {}", day10::part2(&day10input));
    let day11input = day11::parse_input(File::open("day11.input").unwrap());
    println!("day11p1: {}", day11::part1(&day11input));
    println!("day11p2: {}", day11::part2(&day11input));
    let day12input = day12::parse_input(File::open("day12.input").unwrap());
    println!("day12p1: {}", day12::part1(&day12input));
}
