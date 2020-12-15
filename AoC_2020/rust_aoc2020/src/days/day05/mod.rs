use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day05/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn both(input: &str) -> String {
    let mut length = 0;
    let mut min = 1016;
    let mut sum = 0;

    for line in input.lines() {
        length += 1;
        let seat_id = get_seat_id_from_line(line);
        sum += seat_id;

        if seat_id < min {
            min = seat_id
        }
    }

    let max = min + length;

    let mut expected_sum = 0;

    for i in min..(max + 1) {
        expected_sum += i
    }

    let missing_seat = expected_sum - sum;

    return format!("Task 1: {}\nDay 5 Task 2: {y}", max, y = missing_seat);
}

fn get_seat_id_from_line(line: &str) -> u32 {
    let mut bin_string: String = "".to_owned();

    for char in line.chars() {
        if char == 'F' || char == 'L' {
            bin_string.push('0')
        } else {
            bin_string.push('1')
        }
    }

    let parsed = u32::from_str_radix(&*bin_string, 2).expect("Wææ");
    return parsed;
}
