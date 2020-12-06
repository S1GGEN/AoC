use std::fs;
use std::str;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day5/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str) -> String {

    return format!("Task 1: {}", "wææ");
}
pub fn two(input : &str) -> String {

    return format!("Task 1: {}", "wææ");
}

