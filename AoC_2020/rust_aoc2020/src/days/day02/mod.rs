use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day02/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(contents: &String) -> String {
    let mut count = 0;
    for line in contents.lines() {
        let (min, max, match_char, password) = parse_line(line);
        count += check_validity_1(min, max, match_char, password) as u32;
    }

    // println!("Task 1: {}", count);
    return format!("Task 1: {}", count);
}

pub fn two(contents: &String) -> String {
    let mut count = 0;
    for line in contents.lines() {
        let (min, max, match_char, password) = parse_line(line);
        count += check_validity_2(min, max, match_char, password) as u32;
    }

    // println!("Task 2: {}", count);
    return format!("Task 2: {}", count);
}

pub fn both_parts(contents: &String) -> String {
    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in contents.lines() {
        let (min, max, match_char, password) = parse_line(line);
        count_1 += check_validity_1(min, max, match_char, password) as u32;
        count_2 += check_validity_2(min, max, match_char, password) as u32;
    }

    // println!("Task 1: {}", count_1);
    // println!("Task 2: {}", count_2);
    return format!("Task 1: {} \n Task 2: {b}", count_1, b = count_2);
}

fn check_validity_1(min: u32, max: u32, match_char: char, password: &str) -> bool {
    let num_matches = password.matches(match_char).count();
    return num_matches as u32 >= min && num_matches as u32 <= max;
}

fn check_validity_2(min: u32, max: u32, match_char: char, password: &str) -> bool {
    let first_char = password.chars().nth((min - 1) as usize).unwrap();
    let second_char = password.chars().nth((max - 1) as usize).unwrap();

    return (match_char == first_char) ^ (match_char == second_char);
}

fn parse_line(input_string: &str) -> (u32, u32, char, &str) {
    let mut split_iter = input_string.split(|c| c == '-' || c == ':' || c == ' ');

    let min = split_iter.next().unwrap();
    let max = split_iter.next().unwrap();
    let match_char = split_iter.next().unwrap();
    // Skipping because of space after colon
    let password = split_iter.skip(1).next().unwrap();

    return (
        to_int(min),
        to_int(max),
        match_char.chars().nth(0).unwrap(),
        password,
    );
}

fn to_int(string: &str) -> u32 {
    return string.parse::<u32>().unwrap();
}
