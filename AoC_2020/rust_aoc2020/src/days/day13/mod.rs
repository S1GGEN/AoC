use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day13/{}.txt", filename))
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

fn calculate_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let earliest_time = to_int(lines.next().unwrap());
    let bus_ids: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| to_int(x))
        .collect();

    let mut earliest_bus_timestamp = u32::MAX;
    let mut earliest_bus_id = None;

    for bus_id in bus_ids {
        let timestamp = ((earliest_time as f32 / bus_id as f32).ceil() as u32 * bus_id) as u32;
        if timestamp < earliest_bus_timestamp {
            earliest_bus_timestamp = timestamp;
            earliest_bus_id = Some(bus_id);
        }
    }

    Some((earliest_bus_timestamp - earliest_time) * earliest_bus_id.unwrap())
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<u64> {
    let bus_ids: Vec<&str> = input.lines().nth(1).unwrap().split(",").collect();

    let mut bus_offset = 0;

    let mut busses: Vec<(u64, u64)> = vec![]; // (bus_id, offset)

    for bus_id in &bus_ids {
        if bus_id != &"x" {
            busses.push((to_int(bus_id) as u64, bus_offset))
        }
        bus_offset += 1;
    }

    let mut departure_timestamp = 0;
    let mut advancement = 1;

    for bus in &busses {
        let (bus_id, bus_offset) = bus;
        while (departure_timestamp + bus_offset) % bus_id != 0 {
            departure_timestamp += advancement
        }
        advancement *= bus_id
    }

    Some(departure_timestamp)
}

fn to_int(string: &str) -> u32 {
    return string.parse::<u32>().unwrap();
}
