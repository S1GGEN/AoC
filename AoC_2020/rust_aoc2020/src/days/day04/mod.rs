use std::fs;
use std::str;
use regex::Regex;

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day04/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}


pub fn one(input : &String) -> String {
    let mut num_passports = 0;
    let mut invalid_count = 0;
    let passports =  input.split("\n\r");
    for passport in passports {
        num_passports += 1;
        for field in &get_required_fields() {
            if !passport.contains(field) {
                invalid_count += 1;
                break;
            }
        }
    }

    return format!("Task 1: {}", num_passports - invalid_count);
}

pub fn two(input : &String) -> String {
    let mut num_passports = 0;
    let mut invalid_count = 0;
    let passports =  input.split("\n\r");
    for passport in passports {
        num_passports += 1;
        for field in &get_required_fields_2() {
            let (search_term, validation_method) = field;
            let first_match = passport.match_indices(search_term).next();
            if first_match.is_some() {
                let (field_index, _) = passport.match_indices(search_term).next().unwrap();
                let (_, tail) = passport.split_at(field_index + 4);
                if !validation_method(tail.split_whitespace().next().unwrap()) {
                    invalid_count +=1;
                    break;
                }
            } else {
                invalid_count +=1;
                break;
            }

        }
    }

    return format!("Task 2: {}", num_passports - invalid_count);
}


fn get_required_fields() -> [&'static str; 7] {
    return ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

fn validate_byr(value : &str) -> bool {
    return to_int(value) >= 1920 && to_int(value) <= 2002;
}

fn validate_iyr(value : &str) -> bool {
    return to_int(value) >= 2010 && to_int(value) <= 2020;
}

fn validate_eyr(value : &str) -> bool {
    return to_int(value) >= 2020 && to_int(value) <= 2030;
}

fn validate_hgt(value : &str) -> bool {
    if value.len() == 5 && value.ends_with("cm") {
        let (num, _) = value.split_at(3);
        return to_int(num) >= 150 && to_int(num) <= 193;
    } else if value.len() == 4 && value.ends_with("in") {
        let (num, _) = value.split_at(2);
        return to_int(num) >= 59 && to_int(num) <= 76;
    }
    return false;
}

fn validate_hcl(value : &str) -> bool {
    if value.len() == 7 {
        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        return re.is_match(value);
    }
    return false;
}

fn validate_ecl(value : &str) -> bool {
    return get_ecls().iter().any(|v| v == &value);
}

pub fn validate_pid(value : &str) -> bool {
    if value.len() == 9 {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        return re.is_match(value);
    }
    return false;
}

fn get_required_fields_2() -> [(&'static str, fn(&str) -> bool); 7] {
    return [
        ("byr", validate_byr),
        ("iyr", validate_iyr),
        ("eyr", validate_eyr),
        ("hgt", validate_hgt),
        ("hcl", validate_hcl),
        ("ecl", validate_ecl),
        ("pid", validate_pid),
    ];
}

fn get_ecls() -> [&'static str; 7] {
    return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
}

fn to_int(string : &str) -> u32 {
    return string.parse::<u32>().unwrap();
}
