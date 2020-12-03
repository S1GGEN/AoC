use std::fs;
use std::str;

pub fn load_input(filename : &str) -> String {
    let contents = fs::read_to_string(format!("src/days/day1/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return contents;
}

pub fn one(contents : &String) -> String {
    let mut results = "".to_string();

    'outer1: for line in contents.lines(){
        for line2 in contents.lines() {
                if to_int(line) + to_int(line2) == 2020 {
                    results = format!("Task 1 {}",  to_int(line) * to_int(line2));
                    break 'outer1
                }
        }
    }

    return results;
}

pub fn two(contents : &String) -> String {
    let mut results = "".to_string();

    'outer2: for line in contents.lines(){
        for line2 in contents.lines() {
            for line3 in contents.lines() {
                if to_int(line) + to_int(line2) + to_int(line3) == 2020 {
                    results = format!("Task 2 {}",  to_int(line) * to_int(line2) * to_int(line3));
                    break 'outer2
                }
            }
        }
    }

    return results;
}


fn to_int(string : &str) -> u32 {
    return string.parse::<u32>().unwrap();
}