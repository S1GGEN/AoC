use std::{
    fs::File,
    io::{BufRead, BufReader},
    str,
};

pub fn load_input(filename: &str) -> Vec<Vec<bool>> {
    let mut file = File::open(format!("src/days/day03/{}.txt", filename))
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(&mut file);

    let lines: Vec<Vec<bool>> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line").chars().map(|c| bool_from_char(c)).collect())
        .collect();

    return lines;
}

pub fn one(input: &Vec<Vec<bool>>) -> String {
    let most_commons = get_most_commons(input);

    let gamma_string = most_commons.iter().enumerate().fold("".to_string(), |acc, b| acc + if *b.1 {"1"} else {"0"});

    let epsilon_string = gamma_string.chars().fold("".to_string(), |acc, c| acc + (if c == '1' {"0"} else {"1"}));

    let gamma_num: u32 = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_num: u32 = u32::from_str_radix(&epsilon_string, 2).unwrap();

    return format!("Task 1: {}", gamma_num * epsilon_num);
}

fn get_most_commons(input: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut num_input_lines = 0;
    let line_length = input[0].len();

    let mut counts: Vec<u32> = vec![0; line_length];

    for line in input {
        line.iter().enumerate().for_each(|(i, bit)| {
            counts[i] += *bit as u32;
        });
        num_input_lines += 1;
    }

    let most_common: Vec<bool> = counts.iter().enumerate().fold(vec![], |mut acc, count| {
        acc.push(most_least_common(num_input_lines, *count.1, true));
        acc
    });


    return most_common;
}


fn get_count(input: &Vec<Vec<bool>>, bit_position: usize, count_ones: bool) -> u32 {
    input.iter().fold(0, |acc, l| acc + (l[bit_position] == count_ones) as u32)
}

fn most_least_common(num_input_lines: usize, count: u32, most_common: bool) -> bool {
    if most_common {
        return count as f32 >= (num_input_lines as f32/2 as f32);
    }

    return (count as f32) < (num_input_lines as f32/2 as f32);
}

fn bool_from_char(c: char) -> bool {
    c == '1'
}

pub fn two(input: &Vec<Vec<bool>>) -> String {
    // println!("TWO ---------------");
    let oxygen_rating = get_rating(input.to_owned(), 0, true);

    // println!("oxygen_rating: {:#?}", oxygen_rating);
    let oxygen_rating_string = oxygen_rating[0].iter().fold("".to_string(), |acc, b| acc + if *b {"1"} else {"0"});
    let oxygen_rating_num: u32 = u32::from_str_radix(&oxygen_rating_string, 2).unwrap();

    // println!("oxygen_rating_num: {:#?}", oxygen_rating_num);


    let co2_rating = get_rating(input.to_owned(), 0, false);

    // println!("co2_rating: {:#?}", co2_rating);
    let co2_rating_string = co2_rating[0].iter().fold("".to_string(), |acc, b| acc + if *b {"1"} else {"0"});
    let co2_rating_num: u32 = u32::from_str_radix(&co2_rating_string, 2).unwrap();

    // println!("co2_rating_num: {:#?}", co2_rating_num);
    
    return format!("Task 2: {}", oxygen_rating_num * co2_rating_num);
}


fn get_rating(input: Vec<Vec<bool>>, bit_position: usize, is_oxygen: bool) -> Vec<Vec<bool>> {
    if input.len() <= 1 {return input};

    let count = get_count(&input, bit_position, is_oxygen);
    let num_input_lines = input.len();

    let num_match;
    if is_oxygen {
        num_match = (count as f32) >= (num_input_lines as f32/2 as f32);
    } else {
        num_match = !(((count as f32) <= (num_input_lines as f32/2 as f32)) as bool);

    }

    let new_sample : Vec<Vec<bool>> = input.iter().fold(vec![], |mut acc, l| {
        if l[bit_position] == num_match {
            acc.push(l.to_vec());
        };
        acc
    });

    return get_rating(new_sample.to_vec(), bit_position + 1, is_oxygen);
}
