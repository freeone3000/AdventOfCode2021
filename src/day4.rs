use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

const BOARD_SIZE: usize = 5;
const CALLED: i32 = -1;
type Board = [[i32; BOARD_SIZE]; BOARD_SIZE]; //int32_t[5][5]

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
        let mut horiz_win = true;
        let mut vert_win = true;
        for j in 0..BOARD_SIZE {
            horiz_win = horiz_win && (board[i][j] == CALLED);
            vert_win = vert_win && (board[j][i] == CALLED); // note the i,j flip
        }
        if horiz_win || vert_win {
            return true;
        }
    }
    false
}

fn score(last_called: i32, board: &Board) -> i32 {
    let remaining_items = board.iter().fold(0, |state, line | state + line.iter().filter(|x| **x != CALLED).sum::<i32>());
    remaining_items * last_called
}

fn run_game(numbers: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in numbers {
        call_number_on_boards(&number, &mut boards);

        print_boards(&boards); // DEBUG

        match boards.iter().find(|board| is_winner(board)) {
            Some(board) => return score(number, board),
            None => ()
        }
    }
    panic!("No winning board found!");
}

fn run_game_part2(numbers: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in numbers {
        call_number_on_boards(&number, &mut boards);

        if boards.len() == 1 {
            if is_winner(&boards[0]) {
                return score(number, &boards[0]);
            }
        } else {
            boards = boards.into_iter().filter(|board| !is_winner(board)).collect();
        }
    }

    panic!("No winning board found!");
}

fn call_number_on_boards(number: &i32, boards: &mut Vec<Board>) {
    for board in boards.iter_mut() {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if &board[i][j] == number {
                    board[i][j] = CALLED;
                }
            }
        }
    }
}

fn f(filename: &str) -> i32 {
    let (numbers, boards) = parse_file(filename);
    run_game(numbers, boards)
}

pub fn part1() -> i32 {
    assert_eq!(f("day4_sample.txt"), 4512);

    f("day4.txt")
}

pub fn part2() -> i32 {
    let (numbers, boards) = parse_file("day4.txt");
    run_game_part2(numbers, boards)
}