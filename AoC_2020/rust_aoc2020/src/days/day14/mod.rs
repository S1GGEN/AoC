use std::collections::HashMap;
use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day14/{}.txt", filename))
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
    let mut lines = input.lines();

    let mut current_mask;
    let (_, first_mask) = lines.nth(0).unwrap().split_at(7);
    current_mask = first_mask;

    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in lines {
        if line.starts_with("mask") {
            let (_, mask) = line.split_at(7);
            // println!("new mask: {}", mask);
            current_mask = mask;
        } else {
            let (_, tail) = line.split_at(4);
            let mut interested_in = tail.split("] = ");
            let address = to_int(interested_in.next().unwrap());
            let value = to_int(interested_in.next().unwrap());
            let bin_value = format!("{:036b}", value);

            let mut new_bin = "".to_owned();
            for i in 0..current_mask.len() {
                let mask_digit = current_mask.chars().nth(i).unwrap();
                let bin_digit = bin_value.chars().nth(i).unwrap();

                if mask_digit == 'X' {
                    new_bin.push(bin_digit)
                } else {
                    new_bin.push(mask_digit)
                }
            }

            mem.insert(address, u64::from_str_radix(&*new_bin, 2).expect("Oopsie"));
        }
    }

    Some(mem.values().sum::<u64>())
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut current_mask;
    let (_, first_mask) = lines.nth(0).unwrap().split_at(7);
    current_mask = first_mask;

    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in lines {
        if line.starts_with("mask") {
            let (_, mask) = line.split_at(7);
            current_mask = mask;
        } else {
            let (_, tail) = line.split_at(4);
            let mut interested_in = tail.split("] = ");
            let address = to_int(interested_in.next().unwrap());
            let value = to_int(interested_in.next().unwrap());
            let bin_address = format!("{:036b}", address);

            let mut new_bin = "".to_owned();
            for i in 0..current_mask.len() {
                let mask_digit = current_mask.chars().nth(i).unwrap();
                let bin_digit = bin_address.chars().nth(i).unwrap();

                if mask_digit == '1' {
                    new_bin.push('1')
                } else if mask_digit == '0' {
                    new_bin.push(bin_digit)
                } else {
                    // Floating bit, ooooOOooo
                    new_bin.push('X')
                }
            }

            let addresses = get_addresses(new_bin);

            for address in addresses {
                mem.insert(u64::from_str_radix(&*address, 2).expect("Oopsie"), value);
            }
        }
    }
    Some(mem.values().sum::<u64>())
}

fn get_addresses(bin_string: String) -> Vec<String> {
    let mut addresses: Vec<String> = vec![];

    if bin_string.contains("X") {
        let mut replaced_zero = "".to_owned();
        replaced_zero.push_str(&*bin_string.replacen("X", "0", 1));
        let mut replaced_one = "".to_owned();
        replaced_one.push_str(&*bin_string.replacen("X", "1", 1));
        addresses.append(&mut get_addresses(replaced_zero));
        addresses.append(&mut get_addresses(replaced_one))
    } else {
        addresses.push(bin_string);
    }

    addresses
}

fn to_int(string: &str) -> u64 {
    return string.parse::<u64>().unwrap();
}
