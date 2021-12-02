use std::str::FromStr;
use std::ops::AddAssign;
use crate::common;

struct Position {
    depth: i32,
    pos: i32
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.depth += rhs.depth;
        self.pos += rhs.pos;
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut wh_iter = line.split_whitespace();
        let token = wh_iter.next().unwrap();
        let amt = i32::from_str(wh_iter.next().unwrap()).unwrap();
        match token {
            "up" => Ok(Position {
                depth: -amt,
                pos: 0
            }),
            "down" => Ok(Position {
                depth: amt,
                pos: 0
            }),
            "forward" => Ok(Position {
                depth: 0,
                pos: amt
            }),
            _ => Err("Could not determine token type".to_string())
        }
    }
}

fn calculate_offsets(offsets: Vec<Position>) -> Position {
    let mut current  = Position {
        depth: 0,
        pos: 0
    };
    for offset in offsets {
        current += offset;
    }
    current
}

fn parse_line(line: &str) -> Position {
    Position::from_str(line).unwrap()
}

pub fn part1() -> i32{
    let sample_result = calculate_offsets(common::read_file("day2_sample.txt", &parse_line));
    assert_eq!(sample_result.depth * sample_result.pos, 150);

    let result = calculate_offsets(common::read_file("day2.txt", &parse_line));
    return result.depth * result.pos;
}