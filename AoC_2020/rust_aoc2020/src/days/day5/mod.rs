use std::fs;
use std::str;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day5/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn both(input : &str) -> String {
    let mut length = 0;
    let mut min = 1000;
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

    return format!("Task 2-1: {}\nDay 5 Task 2-2: {y}", max, y=missing_seat);
}


fn get_seat_id_from_line(line : &str) -> u32 {
    let binary = line.replace("F", "0").replace("L", "0").replace("R", "1").replace("B", "1");

    let parsed = u32::from_str_radix(&*binary, 2).expect("Wææ");
    return parsed;
}


