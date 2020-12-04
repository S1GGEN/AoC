use std::{
    fs::File,
    str,
    io::{BufRead, BufReader},
};

pub fn load_input(filename : &str) -> Vec<u32> {
    let mut file = File::open(format!("src/days/day1/{}.txt", filename))
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(&mut file);

    let mut lines: Vec<u32> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line").parse().unwrap())
        .collect();

    lines.sort();

    return lines;
}

fn get_target() -> u32 {
    return 2020;
}

pub fn one(lines : &Vec<u32>) -> String {
    let target_sum = get_target();
    let mut from_head = 0;
    let mut from_tail = lines.len() - 1;
    while from_head < from_tail {
        let sum = lines[from_head] + lines[from_tail];

        if sum < target_sum {
            from_head += 1
        } else if sum > target_sum {
            from_tail -=1
        } else {
            return format!("Task 1 {}",  lines[from_head] * lines[from_tail]);
        }

    }

    return format!("Task 1 FAILED!");
}


pub fn two(lines : &Vec<u32>) -> String {
    for (i, &x) in lines.iter().enumerate() {
        let target_sum = get_target() - x;
        let mut from_head = i + 0;
        let mut from_tail = lines.len() - 1;
        while from_head < from_tail {
            let sum = lines[from_head] + lines[from_tail];

            if sum < target_sum {
                from_head += 1
            } else if sum > target_sum {
                from_tail -= 1
            } else {
                return format!("Task 1 {}", x * lines[from_head] * lines[from_tail]);
            }
        }
    }

    return format!("Task 2 FAILED!");
}
