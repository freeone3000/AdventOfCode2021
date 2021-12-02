use std::str::FromStr;
use std::ops::AddAssign;
use crate::common;

struct Position {
    aim: i32,
    pos: i32
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.aim += rhs.aim;
        self.pos += rhs.pos;
    }
}

impl FromStr for Position {
    type Err = String;

    // note that depth is uncal
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut wh_iter = line.split_whitespace();
        let token = wh_iter.next().unwrap();
        let amt = i32::from_str(wh_iter.next().unwrap()).unwrap();
        match token {
            "up" => Ok(Position {
                aim: -amt,
                pos: 0
            }),
            "down" => Ok(Position {
                aim: amt,
                pos: 0
            }),
            "forward" => Ok(Position {
                aim: 0,
                pos: amt
            }),
            _ => Err("Could not determine token type".to_string())
        }
    }
}

fn calculate_offsets(offsets: Vec<Position>) -> Position {
    let mut current  = Position {
        aim: 0,
        pos: 0
    };
    for offset in offsets {
        current += offset;
    }
    current
}

fn calculate_using_aim(offsets: Vec<Position>) -> (Position, i32) {
    //depth is uncalculable as an offset so we have it as a local
    let mut depth = 0;
    let mut state = Position {
        aim: 0,
        pos: 0
    };

    for offset in offsets {
        if offset.pos != 0 { // if we move forward, also increase depth by pos * aim
            depth += offset.pos * state.aim;
        }
        state += offset;
    }
    (state, depth)
}

fn parse_line(line: &str) -> Position {
    Position::from_str(line).unwrap()
}

pub fn part1() -> i32{
    let sample_result = calculate_offsets(common::read_file("day2_sample.txt", &parse_line));
    assert_eq!(sample_result.aim * sample_result.pos, 150);

    let result = calculate_offsets(common::read_file("day2.txt", &parse_line));
    return result.aim * result.pos;
}

pub fn part2() -> i32 {
    let (d_state, d_depth) = calculate_using_aim(common::read_file("day2_sample.txt", &parse_line));
    assert_eq!(d_state.pos, 15);
    assert_eq!(d_depth, 60);

    let (state, depth) = calculate_using_aim(common::read_file("day2.txt", &parse_line));
    state.pos * depth
}