use std::fs;
use std::str;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day9/{}.txt", filename))
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

pub fn calculate_one(input : &str) -> Option<u64> {
    let preamble_size = 25;
    let lines: Vec<u64> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let mut invalid_num = None;

    for i in preamble_size..lines.len() {
        let (_, tail) = lines.split_at(i - preamble_size);
        let (preamble, _) = tail.split_at(preamble_size);
        let num_to_check = lines[i];

        if !is_valid(Vec::from(preamble), num_to_check) {
            invalid_num = Some(num_to_check)
        }
    }

    return invalid_num;
}

fn is_valid(preamble: Vec<u64>, num_to_check : u64) -> bool {
    for i in 0..preamble.len() {
        for j in 0..preamble.len() {
            if i != j {
                if preamble[i] + preamble[j] == num_to_check {
                    return true;
                }
            }
        }
    }

    return false;
}

pub fn two(input : &str, part_one_result : Option<u64>) -> String {
    let result = calculate_two(input, part_one_result);

    if result == None {
        return format!("Task 2: FAILED");
    }
    return format!("Task 2: {}", result.unwrap());
}

fn calculate_two(input : &str, part_one_result : Option<u64>) -> Option<u64> {
    // let target = 675280050; // Kinda cheat, but can save benchmarking time hehe
    let target;

    if part_one_result == None {
        target = calculate_one(input).unwrap();
    } else {
        target = part_one_result.unwrap();
    }

    let lines: Vec<u64> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();


    for search_size in 2..lines.len() {
        for start_index in 0..(lines.len() - search_size) {
            let sum_slice = &lines[start_index..(start_index + search_size)];
            let sum = get_sum(sum_slice);
            if sum == target {
                return Some(get_min_max_sum(sum_slice));
            }
        }
    }
    return None;
}

fn get_sum(nums : &[u64]) -> u64 {
    let mut sum : u64 = 0;

    for num in nums.iter() {
        sum += num;
    }

    return sum;
}

fn get_min_max_sum(nums : &[u64]) -> u64 {
    let mut min = u64::MAX;
    let mut max : u64 = 0;

    for num in nums.iter() {
        if num > &max {
            max = *num;
        }
        if num < &min {
            min = *num;
        }
    }

    return min + max;
}


