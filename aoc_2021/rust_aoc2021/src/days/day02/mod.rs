use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
};

pub fn load_input(filename: &str) -> Vec<(String, u32)> {
    let mut file = File::open(format!("src/days/day02/{}.txt", filename))
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(&mut file);

    let lines: Vec<(String, u32)> = reader
        .lines()
        .map(|l| parse_line(l.expect("Couldn't read a line")))
        .collect();

    return lines;
}

pub fn one(commands: &Vec<(String, u32)>) -> String {
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for command in commands {
        let (dir, value) = command;
        if dir == "forward" {
            horizontal_pos += value;
        } else if dir == "up" {
            depth -= value;
        } else {
            depth += value
        }
    }

    return format!("Task 1: {} * {} = {}", depth, horizontal_pos, depth * horizontal_pos);
}

pub fn two(commands: &Vec<(String, u32)>) -> String {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_pos = 0;

    for command in commands {
        let (dir, value) = command;
        if dir == "forward" {
            horizontal_pos += value;
            depth += aim * value;
        } else if dir == "up" {
            aim -= value;
        } else {
            aim += value
        }
    }

    return format!("Task 2: {} * {} = {}", depth, horizontal_pos, depth * horizontal_pos);
}


fn parse_line(input_string: String) -> (String, u32) {
    let split: Vec<&str> = input_string.split_whitespace().take(2).collect();

    return (split[0].to_string(), split[1].parse().unwrap());
}
