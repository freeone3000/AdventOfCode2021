use std::str::FromStr;
use crate::common;

fn parse_and_unwrap(s: &str) -> i32 {
    i32::from_str(s).unwrap()
}

fn count_increases(items: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last_item = items[0];
    for item in items.iter().skip(1) {
        if item > &last_item {
            count = count + 1;
        }
        last_item = *item;
    }
    count
}


fn count_increases_sliding(items: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last_sum = i32::MAX;
    for item in items.windows(3) {
        let sum = item[0] + item[1] + item[2];
        if sum > last_sum {
            count = count + 1;
        }
        last_sum = sum;
    }
    count
}

pub fn part1() -> i32 {
    let example_depths = common::read_file("day1_sample.txt", &parse_and_unwrap);
    assert_eq!(count_increases(example_depths), 7);

    let depths = common::read_file("day1.txt", &parse_and_unwrap);
    count_increases(depths)
}

pub fn part2() -> i32 {
    let example_depths = common::read_file("day1_sample.txt", &parse_and_unwrap);
    assert_eq!(count_increases_sliding(example_depths), 5);

    let depths = common::read_file("day1.txt", &parse_and_unwrap);
    count_increases_sliding(depths)
}