use std::fs;
use std::str;


pub fn load_input(filename: &str) -> (Vec<u64>, Vec<Vec<Vec<BoardCell>>>) {
    let input = fs::read_to_string(format!("src/days/day04/{}.txt", filename))
        .expect("Something went wrong reading the file");

    let mut split = input.split("\n\n");
    let callouts: Vec<u64> = split.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
    // println!("callouts {:#?}", callouts);

    let boards: Vec<Vec<Vec<BoardCell>>> = split.map(|b| get_board(b)).collect();
    // println!("boards {:#?}", boards);
    
    return (callouts, boards);
}

fn get_board(board: &str) -> Vec<Vec<BoardCell>> {
    return board.split("\n").map(|l| get_board_line(l)).collect();
}

fn get_board_line(board_line: &str) -> Vec<BoardCell> {
    return board_line.split_whitespace().map(|n| {
        return BoardCell {
            cell_value: n.parse().unwrap(),
            is_marked: false,
        }
    }).collect();
}


// Either actually use is_marked or remove this struct
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoardCell {
    cell_value: u64,
    is_marked: bool,
}

fn get_winning_board(callouts: Vec<&u64>, boards: &Vec<Vec<Vec<BoardCell>>>) -> Option<u64> {
    for board in boards {
        // let board = boards[board_index]
        if contains_winning_rows(&callouts, &board) || contains_winning_columns(&callouts, &board) {
            // Calculate sum
            let sum = get_sum(&callouts, &board);
            return Some(sum);
        }
    }

    return None;
}

fn get_sum(callouts: &Vec<&u64>, board: &Vec<Vec<BoardCell>>) -> u64 {
    let sum = board.iter().fold(0, |acc, line| acc + 
        line.iter().fold(0, |line_acc, cell| line_acc + if !callouts.iter().any(|&callout| cell.cell_value == *callout) {cell.cell_value} else {0})
    );
    
    let last_callout = callouts.last().unwrap();

    // println!("sum: {} at last callout: {}, final score: {}", sum, last_callout, sum * **last_callout);
    return sum * **last_callout;
}

fn contains_winning_rows(callouts: &Vec<&u64>, board: &Vec<Vec<BoardCell>>) -> bool {
    for line in board {
        let is_winning = line.iter().all(|&cell| callouts.iter().any(|&callout| cell.cell_value == *callout));
        if is_winning {return true};
    }
    return false;
}

fn contains_winning_columns(callouts: &Vec<&u64>, board: &Vec<Vec<BoardCell>>) -> bool {
    let num_columns = board[0].len();
    let num_rows = board.len();

    for column_index in 0..num_columns {
        let mut is_winning = true;
        for row_index in 0..num_rows {           
            if !callouts.iter().any(|&callout| board[row_index][column_index].cell_value == *callout) {
                is_winning = false;
            }
        }
        if is_winning {return true};
    }
    return false;
}

pub fn one(input: &(Vec<u64>, Vec<Vec<Vec<BoardCell>>>)) -> String {
    let callouts = &input.0;
    let boards = &input.1;

    for callouts_index in 1..(callouts.len() + 1) {
        let sub_callouts: Vec<&u64> = callouts.iter().take(callouts_index).collect();
        let maybe_winning = get_winning_board(sub_callouts, &boards);
        if maybe_winning.is_some() {
            let winning_board_score = maybe_winning.unwrap();
            // println!("winning board score: {} at callouts_index: {}", winning_board_score, callouts_index);
            return format!("Task 2: {}", winning_board_score);
        }
    }
    
    return format!("Task 1: failed");
}



fn get_loosing_board_indices(callouts: &Vec<&u64>, boards: &Vec<Vec<Vec<BoardCell>>>) -> Vec<usize> {
    let mut indices = vec![];
    for board_index in 0..boards.len() {
        let winning_rows = contains_winning_rows(&callouts, &boards[board_index]);
        let winning_columns = contains_winning_columns(&callouts, &boards[board_index]);

        if !winning_rows && !winning_columns {
            indices.push(board_index);
        }
    }

    return indices;
}

pub fn two(input: &(Vec<u64>, Vec<Vec<Vec<BoardCell>>>)) -> String {
    let callouts = &input.0;
    let boards = &input.1;

    let mut loosing_board_index = 0;

    // TODO: Do this recursively with only the loosing boards:
    for callouts_index in 1..(callouts.len() + 1) {
        let sub_callouts: Vec<&u64> = callouts.iter().take(callouts_index).collect();
        let loosing_board_indices = get_loosing_board_indices(&sub_callouts, &boards);

        if loosing_board_indices.len() == 1 {
            loosing_board_index = loosing_board_indices[0];
        } else if loosing_board_indices.len() == 0 {
            let score = get_sum(&sub_callouts, &boards[loosing_board_index]);
            // println!("2: loosing board score: {} (board_index: {}) at callouts_index: {}", score, loosing_board_index, callouts_index);

            return format!("Task 2: {}", score);
        }
    }


    return format!("Task 2: failed");
}

