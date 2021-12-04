use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

const BOARD_SIZE: usize = 5;
type Board = [[i32; BOARD_SIZE]; BOARD_SIZE]; //uint32_t[5][5]

fn print_boards(boards: &Vec<Board>) -> () {
    for i in 0..BOARD_SIZE {
        for board in boards {
            for j in 0..BOARD_SIZE {
                print!("{} ", board[i][j]);
            }
            print!("\t");
        }
        print!("\n");
    }
    println!("----");
}

fn parse_board_line(line: &str) -> [i32; 5] {
    let mut val = [0; 5];
    let mut iter = line.trim_end().split_whitespace();
    for i in 0..5 {
        let str = iter.next().unwrap();
        // println!("Str {} at {}", str, i); // TODO DEBUG
        val[i] = i32::from_str(str).unwrap();
    }
    val
}

fn parse_file(filename: &str) -> (Vec<i32>, Vec<Board>) {
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let number_text = lines.next().unwrap().unwrap();
    let numbers = number_text.trim_end().split(",").map(|x| i32::from_str(x).unwrap()).collect();

    lines.next(); // skip blank lines
    let mut boards = Vec::new();
    'outer: loop {
        let mut board: Board = [[0; 5]; 5];
        // extract to function?
        for i in 0..BOARD_SIZE {
            let line = match lines.next() {
                Some(x) => {
                    match x {
                        Ok(y) => y,
                        _ => break 'outer
                    }
                },
                None => break 'outer
            };
            board[i] = parse_board_line(&line.trim_end());
        }
        boards.push(board);
        lines.next(); // trash blank line
    }
    println!("Board count: {}\n-----", boards.len());
    (numbers, boards)
}

fn is_winner(board: &Board) -> bool {
    for i in 0..BOARD_SIZE {
        let mut horiz_sum = 0;
        let mut vert_sum = 0;
        for j in 0..BOARD_SIZE {
            horiz_sum += board[i][j];
            vert_sum += board[j][i];
        }
        if horiz_sum == 0 || vert_sum == 0 {
            return true;
        }
    }
    false
}

fn score(last_called: i32, board: &Board) -> i32 {
    let remaining_items = board.iter().fold(0, |state, line | state + line.iter().sum::<i32>());
    remaining_items * last_called
}

fn run_game(numbers: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in numbers {
        for board in boards.iter_mut() {
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    if &board[i][j] == &number {
                        board[i][j] = 0;
                    }
                }
            }
        }

        print_boards(&boards); // DEBUG

        match boards.iter().find(|board| is_winner(board)) {
            Some(board) => return score(number, board),
            None => ()
        }
    }
    panic!("No winning board found!");
}

fn f(filename: &str) -> i32 {
    let (numbers, boards) = parse_file(filename);
    run_game(numbers, boards)
}

pub fn part1() -> i32 {
    assert_eq!(f("day4_sample.txt"), 4512);

    f("day4.txt")
}