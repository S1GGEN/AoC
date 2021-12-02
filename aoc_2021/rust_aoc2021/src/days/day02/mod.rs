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
    let (depth, horizontal_pos) = commands.iter().fold((0, 0), |acc, c| {
        if c.0 == "forward" {
            return (acc.0, acc.1 + c.1);
        } else if c.0 == "up" {
            return (acc.0 - c.1, acc.1);
        } else {
            return (acc.0 + c.1, acc.1);
        }
    });

    return format!("Task 1: {} * {} = {}", depth, horizontal_pos, depth * horizontal_pos);
}

pub fn two(commands: &Vec<(String, u32)>) -> String {
    // tuple (depth, horizontal_pos, aim)
    let (depth, horizontal_pos, _) = commands.iter().fold((0, 0, 0), |acc, c| {
        if c.0 == "forward" {
            return (acc.0 + acc.2 * c.1, acc.1 + c.1, acc.2);
        } else if c.0 == "up" {
            return (acc.0, acc.1, acc.2 - c.1);
        } else {
            return (acc.0, acc.1, acc.2 + c.1);
        }
    });

    return format!("Task 2: {} * {} = {}", depth, horizontal_pos, depth * horizontal_pos);
}


fn parse_line(input_string: String) -> (String, u32) {
    let split: Vec<&str> = input_string.split_whitespace().take(2).collect();

    return (split[0].to_string(), split[1].parse().unwrap());
}
