use std::fs;
use std::str;

fn main() {
    let filename = "input.txt";
    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Task 1: {}", get_collision_count(input, 3, 1));

    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut product = 1;
    for slope in slopes.iter() {
        product = product * get_collision_count(input.clone(), slope.0, slope.1)
    }

    println!("Task 2: {}", product);
}

fn get_collision_count(input : String, right_slope : usize, down_slope : usize) -> usize {
    let line_length = 31;
    let mut x_pos = 0;
    let mut count = 0;
    for line in input.lines().step_by(down_slope){
        if line.chars().nth(x_pos).unwrap() == '#' {
            count += 1;
        }
        x_pos = (x_pos + right_slope) % line_length;
    }

    return count;
}
