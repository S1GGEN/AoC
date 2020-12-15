use std::fs;
use std::str;
use std::collections::HashMap;
use std::fs::OpenOptions;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day15/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str) -> String {
    let result = calculate_one(input);

    match result {
        Some(r) => format!("Task 1: {}", r),
        None => "Task 1: FAILED".to_string(),
    }
}


fn calculate_one(input : &str) -> Option<u64> {
   get_nth_spoken(input, 2020)
}

pub fn two(input : &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input : &str) -> Option<u64> {
    // Naive solution, works well enough, but there needs to be a better way to do this
    get_nth_spoken(input, 30000000)
}


fn get_nth_spoken(input : &str, nth : usize) -> Option<u64> {
    let mut starting_numbers : Vec<u64> = input
        .split(",")
        .map(|n| to_int(n))
        .collect();

    let mut number_history : HashMap<u64, Vec<u64>> = HashMap::new();

    for i in 0..starting_numbers.len() {
        let number = starting_numbers[i];
        number_history.insert(number, [i as u64 + 1].to_vec());
    }

    // println!("{:?}", number_history);

    let mut last_spoken = starting_numbers[starting_numbers.len() - 1];
    for i in starting_numbers.len()..nth {
        let to_speak;
        // println!("i: {}", i);

        let last_entry = number_history.get(&last_spoken);

        match last_entry {
            None => {to_speak = 0;}
            _ => {
                let last_entry_unwrapped = last_entry.unwrap();

                match last_entry_unwrapped.len() {
                    1 => {to_speak = 0;}
                    _ => {to_speak = last_entry_unwrapped[1] - last_entry_unwrapped[0];}
                }
            }
        }

        let to_speak_entry = number_history.get(&to_speak);

        match to_speak_entry {
            None => { number_history.insert(to_speak, [i as u64 + 1].to_vec()); }
            _ => {
                let to_speak_entry_unwrapped = to_speak_entry.unwrap();
                number_history.insert(
                    to_speak,
                    [to_speak_entry_unwrapped.last().copied().unwrap(), i as u64 + 1].to_vec()
                );
            }
        }

        last_spoken = to_speak;
    }

    Some(last_spoken)
}


fn to_int(string : &str) -> u64 {
    return string.parse::<u64>().unwrap();
}