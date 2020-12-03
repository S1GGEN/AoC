use std::fs;
use std::str;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");


    'outer1: for line in contents.lines(){
        for line2 in contents.lines() {
                if to_int(line) + to_int(line2) == 2020 {
                    println!("{}",  to_int(line) * to_int(line2));
                    break 'outer1
                }
        }
    }

    'outer2: for line in contents.lines(){
        for line2 in contents.lines() {
            for line3 in contents.lines() {
                if to_int(line) + to_int(line2) + to_int(line3) == 2020 {
                    println!("{}",  to_int(line) * to_int(line2) * to_int(line3));
                    break 'outer2
                }
            }
        }
    }
}

fn to_int(string : &str) -> u32 {
    return string.parse::<u32>().unwrap();
}