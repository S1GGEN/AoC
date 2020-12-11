use std::fs;
use std::str;
use conditional::conditional;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day10/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str) -> String {
    let result = calculate_one(input);

    if result == None {
        return format!("Task 1: FAILED");
    }
    return format!("Task 1: {}", result.unwrap());
}

fn calculate_one(input : &str) -> Option<u32> {
    let mut one_jolt_differences : u32= 0;
    let mut three_jolt_differences: u32 = 1;

    let mut lines: Vec<u32> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    lines.sort();

    for i in 0..lines.len() {
        let mut prev_line = 0;
        if i > 0 {
            prev_line = lines[i - 1]
        }

        if lines[i] - prev_line == 1 {
             one_jolt_differences += 1
        } else if lines[i] - prev_line == 3 {
            three_jolt_differences += 1
        }
    }

    return Some(one_jolt_differences * three_jolt_differences);
}


pub fn two(input : &str) -> String {
    let result = calculate_two(input);

    if result == None {
        return format!("Task 2: FAILED");
    }
    return format!("Task 2: {}", result.unwrap());
}


fn calculate_two(input : &str) -> Option<u64> {
    let mut lines: Vec<u32> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    lines.sort();


    let mut num_permutations: u64 = 1;
    let mut group_length = 0;

    for i in 0..(lines.len() + 1) {
        let diff;

        if i == 0 {
            diff = lines[i]
        } else if i == lines.len() {
            diff = 3
        } else {
            diff = lines[i] - lines[i - 1];
        }

        if diff == 1 {
            group_length += 1
        } else {
            num_permutations *= get_permutations_of_group(group_length) as u64;
            group_length = 0
        }
    }

    return Some(num_permutations);
}

fn get_permutations_of_group(l : u32) -> u32 {
    return u64::pow(2, conditional!(l > 0 ? l - 1 : 0
    )) as u32 - conditional!(l == 4 ? 1 : 0) - conditional!(l == 5 ? 3 : 0);
}
