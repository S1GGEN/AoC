use std::collections::VecDeque;
use std::fs;
use std::str;

pub fn load_input(filename: &str) -> Vec<u64> {
    let input = fs::read_to_string(format!("src/days/day06/{}.txt", filename))
        .expect("Something went wrong reading the file");

    let start_fish: Vec<u8> = input.split(",").map(|f| f.parse().unwrap()).collect();

    let counts = start_fish.iter().fold(vec![0; 9], |mut counts, fish| {
        counts[*fish as usize] += 1;
        counts
    });

    return counts;
}

fn iterate_fish(counts: Vec<u64>, num_days: u16) -> u128 {
    let mut counts = counts.iter().map(|c| *c as u128).collect::<VecDeque<u128>>();

    for _ in 0..num_days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    return counts.iter().sum();
}

pub fn one(counts: &Vec<u64>) -> String {
    let sum  = iterate_fish(counts.to_vec(), 80);
    return format!("Task 1: {}", sum);
}

pub fn two(counts: &Vec<u64>) -> String {
    let sum  = iterate_fish(counts.to_vec(), 256);
    return format!("Task 2: {}", sum);
}

