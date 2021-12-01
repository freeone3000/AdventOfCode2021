use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_file<T>(filename: &str, f: &dyn Fn (&str) -> T) -> Vec<T> {
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);

    let mut res = Vec::new();
    for line in reader.lines() {
        res.push(f(&line.unwrap()));
    }
    res
}