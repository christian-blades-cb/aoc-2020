mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
}
