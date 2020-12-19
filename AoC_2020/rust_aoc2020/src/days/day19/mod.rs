use std::fs;
use std::str;
use regex::Regex;
use std::collections::{HashMap};

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day19/{}.txt", filename))
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
    let mut parts = input.split("\n\n");
    let rules_string = parts.next().unwrap();
    let mut messages = parts.next().unwrap().lines();

    let rules_semi_parsed = get_rules_semi_parsed(rules_string);

    let mut regex = get_regex(&rules_semi_parsed, 0, false);
    let regex_complete = format!("^{}$", regex);


    let re = Regex::new(&*regex_complete).unwrap();
    let mut count = 0;
    for line in messages {
        if re.is_match(line) {
            count += 1
        }
    }

    Some(count)
}


fn get_rules_semi_parsed(rules_string : &str) -> HashMap<u32, Vec<Vec<&str>>> {
    let mut rules= HashMap::new();
    for line in rules_string.lines() {
        let mut index_split = line.split(": ");
        let index = index_split.next().unwrap();
        let different_parts = index_split.next().unwrap().split(" | ");

        let mut line_rules = vec![];

        for part in different_parts {
            let mut part_rules = vec![];
            for rule in part.split(" ") {
                // println!("ææ{}æææ", rule);
                part_rules.push(rule);
            }

            line_rules.push(part_rules);
        }

        rules.insert(to_int(index), line_rules);
    }

    rules
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}


fn calculate_two(input: &str) -> Option<u64> {
    let mut parts = input.split("\n\n");
    let rules_string = parts.next().unwrap();
    let mut messages = parts.next().unwrap().lines();

    let rules_semi_parsed = get_rules_semi_parsed(rules_string);

    let mut regex = get_regex(&rules_semi_parsed, 0, true);
    let regex_complete = format!("^{}$", regex);

    let re = Regex::new(&*regex_complete).unwrap();
    let mut count = 0;
    for line in messages {
        if re.is_match(line) {
            count += 1
        } else {
        }
    }

    Some(count)
}

fn get_11_regex(rules_semi_parsed :  &HashMap<u32, Vec<Vec<&str>>>) -> String {
    // Returns the pattern 11: 42 31 | 42 11 31

    let regex_42 = &*get_regex(rules_semi_parsed, 42, true);
    let regex_31 = &*get_regex(rules_semi_parsed, 31, true);

    let mut head = "".to_owned();
    let mut tail = "".to_owned();

    head.push_str(&*format!("{}", regex_42));
    tail.push_str(&*format!("{}", regex_31));


    // Four iterations is the hacky-whacky minimum requirement for this exact input
    for _ in 0..4 {
        head.push_str(&*format!("({}", regex_42));
        tail = format!("{})?{}", regex_31, tail);
    }


    return format!("{}{}", head, tail);
}


fn get_regex(rules_semi_parsed : &HashMap<u32, Vec<Vec<&str>>>, index: u32, task2 : bool) -> String {
    let these_rules = rules_semi_parsed.get(&index).unwrap();

    return if task2 && index == 11 {
        // 11: 42 31 | 42 11 31
        let my_regex = get_11_regex(rules_semi_parsed);
        my_regex.replace("\"", "")
    } else if task2 && index == 8 {
        // 8: 42 | 42 8
        let regex_42 = &*get_regex(rules_semi_parsed, 42, task2);
        let my_regex = format!("({}+)", regex_42);

        my_regex.replace("\"", "")
    } else {

        let mut parts = vec![];

        for (i, part) in these_rules.iter().enumerate() {
            let mut required = "".to_owned();

            for rule in part {
                if rule.parse::<u32>().is_ok() {
                    let to_push = &*get_regex(rules_semi_parsed, to_int(rule), task2);
                    required.push_str(&*to_push)
                } else {
                    required.push_str(rule);
                }
            }
            let to_push = format!("{}", &*required);
            parts.push(to_push);
        }

        let mut final_regex = "".to_owned();

        if parts.len() == 1 {
            final_regex.push_str(&*parts[0]);
        } else if parts.len() == 2 {
            let to_push = format!("({}|{})", parts[0], parts[1]);
            final_regex.push_str(&*to_push);
        } else if parts.len() == 3 {
            let to_push = format!("({}|{}|{})", parts[0], parts[1], parts[2]);
            final_regex.push_str(&*to_push);
        }

        final_regex.replace("\"", "")
    }
}

fn to_int(string: &str) -> u32 {
    return string.parse::<u32>().unwrap();
}