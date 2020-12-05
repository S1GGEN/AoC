use std::fs;
use std::str;
use math::round;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day5/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str ) -> String {
    let ids = generate_ids(input);

    return format!("Task 1: {}", ids.iter().max().unwrap());
}

pub fn two(input : &String) -> String {
    let mut ids = generate_ids(input);
    ids.sort();

    let mut missing_seat: Option<u32> = None;
    for i in 0..(ids.len() - 1) {
        if ids[i + 1] - ids[i] != 1 {
            missing_seat = Some(ids[i] + 1);
            break;
        }
    }

    return if let Some(m) = &missing_seat {
        format!("Task 2: {}", *m)
    } else {
        format!("Task 2: FAILED")
    }

}



pub fn generate_ids(input : &str) -> Vec<u32> {
    let lines: Vec<u32> =
        input.lines()
            .map(|l | get_seat_id_from_line(l) )
            .collect();
    return lines;
}

fn get_seat_id_from_line(line : &str) -> u32 {
    let (row, column) = get_seat(line);

    return (row * 8) + column;
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
        binary_partition(min, min + round::floor(delta as f64 / 2 as f64, 0) as u32, front_string, tail)
    } else {
        binary_partition(min + round::ceil(delta as f64 / 2 as f64, 0) as u32, max, front_string,tail)
    }
}

fn get_seat(line : &str) -> (u32, u32) {
    let (row_string, column_string) = line.split_at(7);
    let row = binary_partition(0, 127, "F", row_string);
    let column =  binary_partition(0, 7, "L", column_string);

    return (row, column);
}


