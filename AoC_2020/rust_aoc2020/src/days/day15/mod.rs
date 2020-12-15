use std::collections::HashMap;
use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day15/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input: &str) -> String {
    let result = calculate_one(input);

    match result {
        Some(r) => format!("Task 1: {}", r),
        None => "Task 1: FAILED".to_string(),
    }
}

fn calculate_one(input: &str) -> Option<usize> {
    get_nth_spoken(input, 2020)
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<usize> {
    get_nth_spoken(input, 30000000)
}

fn get_nth_spoken(input: &str, nth: usize) -> Option<usize> {
    let starting_numbers: Vec<usize> = input.split(",").map(|n| to_int(n)).collect();

    let mut number_history: HashMap<usize, usize> = HashMap::new();

    for i in 0..starting_numbers.len() {
        let number = starting_numbers[i];
        number_history.insert(number, i + 1);
    }

    let mut last_spoken = starting_numbers[starting_numbers.len() - 1];
    for i in starting_numbers.len()..nth {
        let to_speak;

        let last_entry = number_history.get(&last_spoken);

        match last_entry {
            None => {
                to_speak = 0
            }
            _ => {
                let last_entry_unwrapped = last_entry.unwrap();
                to_speak = i - last_entry_unwrapped;
            }
        }

        number_history.insert(last_spoken, i);

        last_spoken = to_speak;
    }

    Some(last_spoken)
}

fn to_int(string: &str) -> usize {
    return string.parse::<usize>().unwrap();
}
