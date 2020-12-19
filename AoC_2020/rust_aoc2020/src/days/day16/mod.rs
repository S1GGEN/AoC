use std::collections::HashMap;
use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day16/{}.txt", filename))
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

fn get_invalid_nums(
    nearby_tickets: Vec<Vec<u16>>,
    fields: HashMap<&str, Vec<Vec<u16>>>,
) -> Vec<u16> {
    let mut invalid_numbers: Vec<u16> = vec![];
    for ticket in nearby_tickets {
        for num in ticket {
            let mut valid = false;
            for rule in fields.values() {
                if position_is_valid(num, rule) {
                    valid = true;
                }
            }
            if valid == false {
                invalid_numbers.push(num)
            }
        }
    }

    invalid_numbers
}

fn calculate_one(input: &str) -> Option<u16> {
    let mut input_blocks = input.split("\n\n");
    let fields_and_rules = get_fields(input_blocks.next().unwrap());
    let nearby_tickets = get_nearby_tickets(input_blocks.skip(1).next().unwrap()); // Skipping "your ticket:"

    let invalid_numbers = get_invalid_nums(nearby_tickets, fields_and_rules);

    Some(invalid_numbers.iter().sum())
}

fn position_is_valid(num: u16, rule: &Vec<Vec<u16>>) -> bool {
    for range in rule {
        if num >= range[0] && num <= range[1] {
            return true;
        }
    }
    return false;
}

fn get_fields(rule_input: &str) -> HashMap<&str, Vec<Vec<u16>>> {
    let mut rules: HashMap<&str, Vec<Vec<u16>>> = HashMap::new();
    for line in rule_input.lines() {
        let mut split = line.split(": ");
        let name = split.next();
        let ranges = split
            .next()
            .unwrap()
            .split(" or ")
            .map(|range| {
                let mut upper_lower = range.split("-");
                [
                    to_int(upper_lower.next().unwrap()),
                    to_int(upper_lower.next().unwrap()),
                ]
                .to_vec()
            })
            .collect();
        rules.insert(name.unwrap(), ranges);
    }

    rules
}

fn get_nearby_tickets(input_block: &str) -> Vec<Vec<u16>> {
    let mut lines = input_block.lines();
    lines.next(); // Skipping first line "nearby tickets:"
    let mut tickets = vec![];

    while let Some(line) = lines.next() {
        let ticket_line = line.split(",").fold(vec![], |mut acc, num| {
            acc.push(to_int(num));
            acc
        });
        tickets.push(ticket_line);
    }
    tickets
}

fn get_my_ticket(input_block: &str) -> &str {
    let mut lines = input_block.lines();

    lines.nth(1).unwrap()
}

fn get_number_of_columns(my_ticket: &str) -> usize {
    let length = my_ticket.split(",").collect::<Vec<&str>>().len();

    length
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<u64> {
    let mut input_blocks = input.split("\n\n");
    let fields_and_rules = get_fields(input_blocks.next().unwrap());
    let my_ticket = get_my_ticket(input_blocks.next().unwrap());
    let nearby_tickets = get_nearby_tickets(input_blocks.next().unwrap());
    let valid_tickets = get_valid_tickets(nearby_tickets, &fields_and_rules);

    let num_columns = get_number_of_columns(my_ticket);
    let mut available_fields: Vec<&str> = fields_and_rules.keys().cloned().collect::<Vec<&str>>();
    let mut unassigned_columns: Vec<usize> = (0..num_columns).collect::<Vec<usize>>();
    let mut order: HashMap<&str, usize> = HashMap::new();

    while available_fields.len() > 0 {
        for i in 0..unassigned_columns.len() {
            let unassigned_column = unassigned_columns[i];
            let mut possible_fields = vec![];
            for field_name in &*&available_fields {
                if column_valid(
                    &valid_tickets,
                    &fields_and_rules,
                    &unassigned_column,
                    field_name,
                ) {
                    possible_fields.push(field_name)
                }
            }
            if possible_fields.len() == 1 {
                order.insert(possible_fields[0], unassigned_column);
                let field_index = available_fields
                    .iter()
                    .position(|f_n| f_n == possible_fields[0])
                    .unwrap();
                available_fields.remove(field_index);
                let column_index = unassigned_columns
                    .iter()
                    .position(|u_c| u_c == &unassigned_column)
                    .unwrap();
                unassigned_columns.remove(column_index);
                break;
            }
        }
    }

    let mut result: u64 = 1;
    for assignment in &order {
        let (key, value) = assignment;
        let ticket_values: Vec<&str> = my_ticket.split(",").collect();

        if key.starts_with("departure") {
            result *= to_int(ticket_values[*value]) as u64;
        }
    }
    Some(result)
}

fn column_valid(
    valid_tickets: &Vec<Vec<u16>>,
    fields_and_rules: &HashMap<&str, Vec<Vec<u16>>>,
    column_index: &usize,
    field_name: &str,
) -> bool {
    for i in 0..valid_tickets.len() {
        let num = valid_tickets[i][*column_index];
        if !position_is_valid(num, fields_and_rules.get(field_name).unwrap()) {
            return false;
        }
    }
    true
}

fn get_valid_tickets(
    nearby_tickets: Vec<Vec<u16>>,
    fields: &HashMap<&str, Vec<Vec<u16>>>,
) -> Vec<Vec<u16>> {
    let mut valid_tickets: Vec<Vec<u16>> = nearby_tickets.clone();

    let mut num_removed = 0;
    for (index, ticket) in nearby_tickets.iter().enumerate() {
        let mut ticket_valid = true;
        for num in ticket {
            let mut num_invalid = true;
            for rule in fields.values() {
                if position_is_valid(*num, rule) {
                    num_invalid = false;
                }
            }
            if num_invalid == true {
                ticket_valid = false;
                break;
            }
        }
        if ticket_valid == false {
            valid_tickets.remove(index - num_removed);
            num_removed += 1;
        }
    }

    valid_tickets
}

fn to_int(string: &str) -> u16 {
    return string.parse::<u16>().unwrap();
}
