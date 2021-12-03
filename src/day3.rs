use crate::common;
use std::collections::HashMap;

fn num_to_digits(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn binary_chars_to_number(bchar : &Vec<char>) -> i32 {
    let mystr : String = bchar.iter().collect();
    i32::from_str_radix(&mystr, 2).unwrap()
}

fn parse_numbers(lines: Vec<Vec<char>>) -> i32 {
    let mut gamma : Vec<char> = Vec::new();
    let mut epsilon : Vec<char> = Vec::new();

    let max = lines[0].len(); // all sizes
    gamma.resize(max, '\0');
    epsilon.resize(max, '\0');
    let mut freq_table: HashMap<char, i32> = HashMap::new();
    for i in 0..max {
        for line in &lines {
            freq_table.insert(line[i], freq_table.get(&line[i]).unwrap_or(&0) + 1);
        }
        let mut hash_vec : Vec<(&char, &i32)> = freq_table.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(&a.1));
        gamma[i] = hash_vec[0].0.clone();
        epsilon[i] = hash_vec[hash_vec.len() - 1].0.clone();

        freq_table.clear();
    }
    binary_chars_to_number(&gamma) * binary_chars_to_number(&epsilon)
}

pub fn part1() -> i32 {
    let run = |s| parse_numbers(common::read_file(s, &num_to_digits));
    assert_eq!(run("day3_sample.txt"), 198);

    run("day3.txt")
}