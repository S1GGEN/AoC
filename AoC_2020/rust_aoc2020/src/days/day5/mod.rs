use std::fs;
use std::str;
use math::round;
use std::cmp::Reverse;
use std::vec::IntoIter;
use linked_list::LinkedList;
use std::iter::FromIterator;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day5/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}


pub fn one(input : &String) -> String {

    let mut highest_seat_id = 0;

    for line in input.lines() {
        let seat_id = get_seat_id_from_line(line);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id
        }
    }

    return format!("Task 1: {}", highest_seat_id);
}

fn get_seat_id_from_line(line : &str) -> u32 {
    let (row, column) = get_seat(line);

    return (row * 8) + column;
}

fn get_seat(line : &str) -> (u32, u32) {
    let (row_string, column_string) = line.split_at(7);
    let row = binary_partition(0, 127, "F", row_string);
    let column =  binary_partition(0, 7, "L", column_string);

    return (row, column);
}

pub fn binary_partition(min : u32, max: u32, front_string : &str, position_string : &str) -> u32 {
    let (head, tail) = position_string.split_at(1);

    if position_string.len() == 1 {
        return if head == front_string {
            min
        } else {
            max
        }
    }

    let delta = max - min;
    return if head == front_string {
        binary_partition(min, min + round::floor(delta as f64 / 2 as f64, 0) as u32, front_string, tail)
    } else {
        binary_partition(min + round::ceil(delta as f64 / 2 as f64, 0) as u32, max, front_string,tail)
    }
}

pub fn two(input : &String) -> String {
    let mut lines = LinkedList::from_iter(sort_lines(input));
    let mut c = lines.cursor();

    let missing_seat;


    let (mut last_row, _) = c.next().unwrap().split_at(7);

    // To take care of the first row with missing seats:
    loop {
        let (this_row, _) = c.next().unwrap().split_at(7);
        if last_row != this_row {
            last_row = this_row;
            break;
        }
        last_row = this_row;
    }

    // Main loop
    loop {

        c.seek_forward(6);
        let (this_row, _) = c.next().unwrap().split_at(7);

        if this_row != last_row {
            // That row we flew past contained the missing seat, we need to backtrack and take a closer look!

            c.prev();
            let mut prev_seat_id = get_seat_id_from_line(c.prev().unwrap());

            loop {
                let pre_prev_seat_id = get_seat_id_from_line(c.prev().unwrap());
                if pre_prev_seat_id - prev_seat_id != 1 {
                    // FOUND MISSING SEAT
                    missing_seat = prev_seat_id + 1;
                    break;
                }
                prev_seat_id = pre_prev_seat_id;

            }


            break;
        }

        let (this_row, _) = c.next().unwrap().split_at(7);
        last_row = this_row;
    }

    return format!("Task 2: {}", missing_seat);
}


fn sort_lines(input : &String) -> IntoIter<&str>{
    let mut lines: Vec<&str> = input.lines()
        .collect();

    lines.sort_by_key(|w| Reverse(*w));

    let iterable = lines.into_iter();
    return iterable;
}
