use std::fs;
use std::str;
use std::collections::HashSet;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day08/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str) -> String {
    let lines : Vec<&str> = input.lines().collect();
    let (acc_value, _) = calculate(lines, None::<i32>);
    return format!("Task 1: {}", acc_value);
}


fn calculate(lines : Vec<&str>, flip_line : Option<i32>) -> (i32, i32) {
    let mut acc : i32 = 0;
    let mut i: i32 = 0;

    let mut visited_lines : HashSet<i32> =  HashSet::new();

    while i < lines.len() as i32 {
        if visited_lines.contains(&i) {
            break;
        }
        visited_lines.insert(i);

        let line : &str = lines[i as usize];
        let (mut line_type, _) = line.split_at(3);

        if flip_line != None && i == flip_line.unwrap() {
            if line_type == "jmp" {
                line_type = "nop";
            } else if line_type == "nop" {
                line_type = "jmp";
            }
        }

        if line_type == "acc" {
            let (_, value) = line.split_at(4);
            acc += value.parse::<i32>().unwrap();
        }
        if line_type == "jmp" {
            let (_, value) = line.split_at(4);
            i += value.parse::<i32>().unwrap();
        } else {
            i += 1;
        }
    }

    return (acc, i - 1);
}


pub fn two(input : &str) -> String {
    let lines : Vec<&str> = input.lines().collect();
    let mut final_acc = None;

    for i in 0..lines.len() {
        let my_lines : Vec<&str> = lines.iter().cloned().collect();
        let (acc_value, termination_line) = calculate(my_lines, Some(i as i32));
        if termination_line == lines.len() as i32 - 1 {
            // We did it boys
            final_acc = Some(acc_value);
            break;
        }
    }
    if final_acc == None {
        return format!("Task 2: FAILED");
    }
    return format!("Task 2: {}", final_acc.unwrap());
}

