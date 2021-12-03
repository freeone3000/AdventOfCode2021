use std::env;
use std::str::FromStr;

mod common;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = i32::from_str(&args[1]).unwrap();
    if day == 1 {
        println!("Day 1 part 1: {}", day1::part1());
        println!("Day 1 part 2: {}", day1::part2());
    } else if day == 2 {
        println!("Day 2 part 1: {}", day2::part1());
        println!("Day 2 part 1: {}", day2::part2());
    } else if day == 3 {
        println!("Day 3 part 1: {}", day3::part1());
        println!("Day 3 part 2: {}", day3::part2());
    }
}
