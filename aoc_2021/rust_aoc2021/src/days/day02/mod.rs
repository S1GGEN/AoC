use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day02/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(contents: &String) -> String {
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for line in contents.lines() {
        let (dir, value) = parse_line(line);
        if dir == "forward" {
            horizontal_pos += value;
        } else if dir == "up" {
            depth -= value;
        } else {
            depth += value
        }
    }

    // println!("Task 1: {}", count);
    return format!("Task 1: {} * {} = {}", depth, horizontal_pos, depth * horizontal_pos);
}

pub fn two(contents: &String) -> String {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for line in contents.lines() {
        let (dir, value) = parse_line(line);
        if dir == "forward" {
            horizontal_pos += value;
            depth += aim * value;
        } else if dir == "up" {
            aim -= value;
        } else {
            aim += value
        }
    }

    // println!("Task 1: {}", count);
    return format!("Task 2: {} * {} = {}", depth, horizontal_pos, depth * horizontal_pos);
}

fn parse_line(input_string: &str) -> (&str, u32) {
    let split: Vec<&str> = input_string.split_whitespace().take(2).collect();

    return (split[0], split[1].parse().unwrap());
}
