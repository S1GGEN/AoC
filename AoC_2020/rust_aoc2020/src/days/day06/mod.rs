use std::fs;
use std::str;
use std::collections::{HashSet, HashMap};

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day06/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str) -> String {
    let groups = input.split("\n\r");
    let mut sum = 0;

    for group in groups {
        sum += find_any_count(group);
    }

    return format!("Task 1: {}", sum);
}

pub fn two(input : &str) -> String {
    let groups = input.split("\n\r");
    let mut sum = 0;

    for group in groups {
        sum += find_everyone_count(group);
    }


    return format!("Task 2: {}", sum);
}

fn find_any_count(group : &str) -> u32 {
    let filtered : String = group.chars().filter(|c| !c.is_whitespace()).collect();
    let unique: HashSet<_> = filtered.chars().collect();

    return unique.len() as u32;
}

fn find_everyone_count(group : &str) -> u32 {
    let mut frequency: HashMap<char, u32> = HashMap::new();

    let mut length: u32 = 0;
    for person in group.lines() {
        length += 1;
        for question in person.chars() {
            *frequency.entry(question).or_insert(0) += 1;
        }
    }
    length -= 1; // Hacky, but it works
    frequency.retain(|_, f| f >= &mut length);

    return frequency.len() as u32;
}

