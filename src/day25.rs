use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use array2d::Array2D;

const EAST_FACING: char  = '>';
const SOUTH_FACING: char = 'v';
const EMPTY: char = '.';

fn parse_input(filename: &Path) -> Array2D<char> {
    let mut items: Vec<Vec<char>> = Vec::new();
    let file = File::open(&filename).unwrap();
    for line in BufReader::new(file).lines() {
        let line_chars = line.unwrap().chars().collect::<Vec<_>>();
        items.push(line_chars);
    }

    Array2D::from_rows(&*items)
}

fn next_step(prev_step: &Array2D<char>) -> Array2D<char> {
    let max_row = prev_step.num_rows();
    let max_col = prev_step.num_columns();
    let mut east_result = Array2D::filled_with(EMPTY, max_row, max_col);

    //east-facing herd moves first, then south-facing herd
    for row in 0..max_row {
        for col in 0..max_col {
            if prev_step[(row, col)] == EAST_FACING {
                if col + 1 < max_col && prev_step[(row, col+1)] == EMPTY { // can be moved
                    // east_result[(row, col)] = EMPTY; // NOTE: Empty is the default value
                    east_result[(row, col + 1)] = EAST_FACING;
                } else if col + 1 == max_col && prev_step[(row, 0)] == EMPTY { // can be moved ("strong currents")
                    // east_result[(row, col)] = EMPTY;
                    east_result[(row, 0)] = EAST_FACING;
                } else { // cannot be moved
                    east_result[(row, col)] = EAST_FACING;
                }
            } else if prev_step[(row, col)] == SOUTH_FACING { // copy over south
                east_result[(row, col)] = SOUTH_FACING
            }
        }
    }

    // doing another array, to prove correctness over efficiency. we might be able to do this with one fewer alloc
    let mut south_result = Array2D::filled_with(EMPTY, max_row, max_col);
    for row in 0..max_row {
        for col in 0..max_col {
            if east_result[(row, col)] == EAST_FACING { // copy over east (in same order as above loop)
                south_result[(row, col)] = EAST_FACING
            } else if east_result[(row, col)] == SOUTH_FACING {
                if row + 1 < max_row && east_result[(row+1, col)] == EMPTY { // can be moved
                    // south_result[(row, col)] = EMPTY; // NOTE: Empty is the default value
                    south_result[(row + 1, col)] = SOUTH_FACING;
                } else if row + 1 == max_row && east_result[(0, col)] == EMPTY { // can be moved ("strong currents")
                    // south_result[(row, col)] = EMPTY;
                    south_result[(0, col)] = SOUTH_FACING;
                } else { // can't be moved
                    south_result[(row, col)] = SOUTH_FACING;
                }
            }
        }
    }

    // south is our final movement, so return that
    south_result
}

// fn print_step(step: &Array2D<char>) {
//     for row in 0..step.num_rows() {
//         for col in 0..step.num_columns() {
//             print!("{}", step[(row, col)]);
//         }
//         println!();
//     }
// }

pub fn part1() -> i32 {
    let mut cur_step = parse_input(Path::new("day25.txt"));
    let mut stepno = 0;
    loop {
        stepno += 1;
        // print_step(&cur_step);
        // println!();

        let next_step = next_step(&cur_step);
        if next_step == cur_step {
            break;
        }
        cur_step = next_step;
    }
    stepno
}