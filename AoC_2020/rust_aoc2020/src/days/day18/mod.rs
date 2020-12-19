use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day18/{}.txt", filename))
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

fn calculate_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let line_stripped = line
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let queue = line_stripped.as_str().chars().collect::<Vec<char>>();
        sum += calculate_line(&queue);
    }

    Some(sum)
}

fn calculate_line(queue: &Vec<char>) -> u64 {
    let (mut acc, mut i) = assert_value(queue, 0);
    while i < queue.len() as u32 {
        if queue[i as usize] == '+' {
            let (new_value, new_i) = assert_value(&queue, i + 1);
            acc += new_value;
            i = new_i;
        } else {
            // if queue[i as usize] == '*'
            let (new_value, new_i) = assert_value(&queue, i + 1);
            acc *= new_value;
            i = new_i;
        }
    }
    acc
}

fn assert_value(queue: &Vec<char>, i: u32) -> (u64, u32) {
    let mut i = i;

    return if queue[i as usize] == '(' {
        let start = i + 1;
        i += 1;
        let mut open = 1;
        while i < queue.len() as u32 && open > 0 {
            if queue[i as usize] == '(' {
                open += 1
            } else if queue[i as usize] == ')' {
                open -= 1
            }
            i += 1
        }
        let sub_line = &queue[start as usize..i as usize - 1].to_vec();
        let value = calculate_line(sub_line);

        (value, i)
    } else {
        (to_int(queue[i as usize]), i + 1)
    };
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let line_stripped = line
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let queue = line_stripped.as_str().chars().collect::<Vec<char>>();
        sum += calculate_line_2(&queue);
    }

    Some(sum)
}

fn calculate_line_2(queue: &Vec<char>) -> u64 {
    let (mut acc, mut i) = assert_value_2(queue, 0);
    while i < queue.len() as u32 {
        if queue[i as usize] == '+' {
            let (new_value, new_i) = assert_value_2(&queue, i + 1);
            acc += new_value;
            i = new_i;
        } else {
            // if queue[i as usize] == '*'
            let (new_value, new_i) = assert_value_2(&queue, i + 1);
            acc *= new_value;
            i = new_i;
        }
    }
    acc
}

fn assert_value_2(queue: &Vec<char>, i: u32) -> (u64, u32) {
    let mut i = i;

    return if i > 0 && queue[i as usize - 1] == '*' {
        let start = i;
        let mut open = 0;
        while i < queue.len() as u32 {
            if open <= 0 && queue[i as usize] == '*' {
                break;
            }
            if queue[i as usize] == '(' {
                open += 1
            } else if queue[i as usize] == ')' {
                open -= 1
            }
            i += 1
        }
        let sub_line = &queue[start as usize..i as usize].to_vec();
        let value = calculate_line_2(sub_line);

        (value, i)
    } else if queue[i as usize] == '(' {
        let start = i + 1;
        i += 1;
        let mut open = 1;
        while i < queue.len() as u32 && open > 0 {
            if queue[i as usize] == '(' {
                open += 1
            } else if queue[i as usize] == ')' {
                open -= 1
            }
            i += 1
        }
        let sub_line = &queue[start as usize..i as usize - 1].to_vec();
        let value = calculate_line_2(sub_line);

        (value, i)
    } else {
        (to_int(queue[i as usize]), i + 1)
    };
}

fn to_int(char: char) -> u64 {
    return char.to_digit(10).unwrap() as u64;
}
