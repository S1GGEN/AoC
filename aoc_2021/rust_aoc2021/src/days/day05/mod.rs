use core::ops::RangeInclusive;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
};

pub fn load_input(filename: &str) -> Vec<((u32, u32), (u32, u32))> {
    let mut file = File::open(format!("src/days/day05/{}.txt", filename))
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(&mut file);

    let lines: Vec<((u32, u32), (u32, u32))> = reader
        .lines()
        .map(|l| to_line(l.expect("Couldn't read a line")))
        .collect();

    return lines;
}

fn to_line(line: String) -> ((u32, u32), (u32, u32)) {
    let split: Vec<&str> = line.split(" -> ").collect();
    
    return (to_coord(split[0]), to_coord(split[1]));
}

fn to_coord(str_coord: &str) -> (u32, u32) {
    let coords: Vec<u32> = str_coord.split(",").map(|num| num.parse().unwrap()).collect();

    return (coords[0], coords[1]);
}


fn get_2d_lines(lines: &Vec<((u32, u32), (u32, u32))>) -> Vec<((u32, u32), (u32, u32))> {
    let two_d_lines = lines.iter().filter_map(|l| {
        if l.0.0 == l.1.0 || l.0.1 == l.1.1 {
            return Some(*l)
        }
        return None;
    }).collect();

    return two_d_lines;
}

fn calculate_locations_hor_ver(two_d_lines: Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut locations: HashMap<(u32, u32), u16> = HashMap::new();
    
    for line in two_d_lines {
        if line.0.0 == line.1.0 {
            // horizontal
            for y in get_range_inclusive(line.0.1, line.1.1) {
                let counter = locations.entry((line.0.0, y)).or_insert(0);
                *counter += 1;
            }
        } else {
            // vertical
            for x in get_range_inclusive(line.0.0, line.1.0) {
                let counter = locations.entry((x, line.0.1)).or_insert(0);
                *counter += 1;
            }
        }
    }

    let count = locations.values().fold(0, |acc, l| acc + (*l >= 2) as u32);

    return count;
}

fn get_range_inclusive(from: u32, to: u32) -> RangeInclusive<u32> {
    if from < to {
        return from..=to;
    }

    return to..=from;
}

fn get_offset_range_inclusive(from: i32, to: i32) -> RangeInclusive<u32> {
    if from > to {
        return 0..=(from- to) as u32;
    }

    return 0..=(to-from) as u32;
}

fn get_offset(from: u32, to: u32) -> i32 {
    return (to as i32) - (from as i32);
}

fn calculate_locations_hor_ver_dia(lines: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    let mut locations: HashMap<(u32, u32), u16> = HashMap::new();
    
    for line in lines {
            let offset_x = get_offset(line.0.0, line.1.0);
            let offset_y = get_offset(line.0.1, line.1.1);

            let range;
            if offset_x != 0 {
                range = get_offset_range_inclusive(0, offset_x);
            } else {
                range = get_offset_range_inclusive(0, offset_y); 
            }
        
            for offset in range {
                let x: i32 = line.0.0 as i32 + if offset_x != 0 {if offset_x < 0 {-1 * offset as i32} else {offset as i32}} else {0};
                let y: i32 = line.0.1 as i32 + if offset_y != 0 {if offset_y < 0 {-1 * offset as i32} else {offset as i32}} else {0};
                
                let counter = locations.entry((x as u32, y as u32)).or_insert(0);
                *counter += 1;
            }
    }

    let count = locations.values().fold(0, |acc, l| acc + (*l >= 2) as u32);

    return count;
}

pub fn one(input: &Vec<((u32, u32), (u32, u32))>) -> String {
    let two_d_lines = get_2d_lines(input);
    let count = calculate_locations_hor_ver(two_d_lines);

    return format!("Task 1: {}", count);
}

pub fn two(input: &Vec<((u32, u32), (u32, u32))>) -> String {
    let count = calculate_locations_hor_ver_dia(input);
    
    return format!("Task 2: {}", count);
}

