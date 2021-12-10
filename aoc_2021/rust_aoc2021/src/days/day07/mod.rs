use std::fs;
use std::str;


pub fn load_input(filename: &str) -> Vec<u64> {
    let input = fs::read_to_string(format!("src/days/day07/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input.split(",").map(|f| f.parse().unwrap()).collect();
}

fn find_cheapest_position_1(initial_positions: &Vec<u64>) -> u64 {
    let min_pos = initial_positions.iter().min().unwrap();
    let max_pos = initial_positions.iter().max().unwrap();

    let mut cheapest_consumption = u64::MAX;

    for try_position in (*min_pos)..=(*max_pos) {
        let fuel = initial_positions.iter().fold(0, |acc, initial_position | acc + (*initial_position as i64 - try_position as i64).abs() as u64);
        if fuel < cheapest_consumption {
            cheapest_consumption = fuel;
        }
    }

    return cheapest_consumption;
}

fn find_cheapest_position_2(initial_positions: &Vec<u64>) -> u64 {
    let min_pos = initial_positions.iter().min().unwrap();
    let max_pos = initial_positions.iter().max().unwrap();

    let mut cheapest_consumption = u64::MAX;

    for try_position in (*min_pos)..=(*max_pos) {
        let fuel = initial_positions.iter().fold(0, |acc, initial_position | {
            let pos_diff = (*initial_position as i64 - try_position as i64).abs() as u64;
            let cost = pos_diff*(pos_diff+1)/2;
            acc + cost
            }
        );
            
        if fuel < cheapest_consumption {
            cheapest_consumption = fuel;
        }
    }

    return cheapest_consumption;
}

pub fn one(input: &Vec<u64>) -> String {
    let cheapest_position = find_cheapest_position_1(input);

    return format!("Task 1: {}", cheapest_position);
}

pub fn two(input: &Vec<u64>) -> String {
    let cheapest_position = find_cheapest_position_2(input);

    return format!("Task 2: {}", cheapest_position);
}

