use std::fs;
use std::str;
use math::round;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day5/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}


pub fn one(input : &String) -> String {

    let mut highest_seat_id = 0;

    for line in input.lines() {
        let (row_string, column_string) = line.split_at(7);
        let row = binary_partition(0, 127, "F", row_string);
        let column =  binary_partition(0, 7, "L", column_string);

        let seat_id = row * 8 + column;
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id
        }
    }

    return format!("Task 1: {}", highest_seat_id);
}

pub fn binary_partition(min : u32, max: u32, front_string : &str, position_string : &str) -> u32 {
    let (head, tail) = position_string.split_at(1);

    if position_string.len() == 1 {
        return if head == front_string {
            min
        } else {
            max
        }
    }

    let delta = max - min;
    return if head == front_string {
        binary_partition(min, min + round::floor((delta as f64 / 2 as f64), 0) as u32, front_string, tail)
    } else {
        binary_partition(min + round::ceil((delta as f64 / 2 as f64), 0) as u32, max, front_string,tail)
    }
}

pub fn two(input : &String) -> String {

    return format!("Task 2: {}", "aaa");
}
