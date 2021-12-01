use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
};

pub fn load_input(filename: &str) -> Vec<u32> {
    let mut file = File::open(format!("src/days/day01/{}.txt", filename))
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(&mut file);

    let lines: Vec<u32> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line").parse().unwrap())
        .collect();

    return lines;
}

pub fn one(lines: &Vec<u32>) -> String {
    let count = lines.windows(2).fold(0, |acc, w| acc + (w[0] < w[1]) as u32);

    return format!("Task 1 {}", count);
}

pub fn two(lines: &Vec<u32>) -> String {

    let count = lines.windows(4).fold(0, |acc, w| acc + (w[0] + w[1] + w[2] < w[1] + w[2] + w[3]) as u32);

    return format!("Task 2 {}", count);

    // return format!("Task 2 FAILED!");
}
