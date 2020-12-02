use std::fs;
use std::str;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut count_1 = 0;
    let mut count_2 = 0;
    for line in contents.lines(){
        let (min, max, match_char, password) = get_values(line);

        if check_validity_1(min, max, match_char, password) {
            count_1 += 1;
        }
        if check_validity_2(min, max, match_char, password) {
            count_2 += 1;
        }
    }

    println!("count_1: {}", count_1);
    println!("count_2: {}", count_2);
}


fn check_validity_1(min : u32, max : u32, match_char : char, password : &str) -> bool {
    let num_matches = password.matches(match_char).count();
    // println!("num_matches: {}", num_matches);
    return num_matches as u32 >= min && num_matches as u32 <= max;
}


fn check_validity_2(min : u32, max : u32, match_char : char, password : &str) -> bool {
    let first_char = password.chars().nth((min - 1) as usize).unwrap();
    let second_char = password.chars().nth((max - 1) as usize).unwrap();

    return
        !(match_char == first_char && match_char == second_char)
        && ((match_char == first_char) || (match_char ==  second_char));
}


fn get_values(input_string : &str) -> (u32, u32, char, &str) {
    let mut split_iter = input_string.split(|c| c == '-' || c == ':' || c == ' ');
    let min : &str = split_iter.next().unwrap();

    let max = split_iter.next().unwrap();

    let match_char = split_iter.next().unwrap();

    // TODO: Find out if I can avoid this line:
    let _stupid_space = split_iter.next();

    let password = split_iter.next().unwrap();

   return (to_int(min), to_int(max), match_char.chars().nth(0).unwrap(), password);
}


fn to_int(string : &str) -> u32 {
    return string.parse::<u32>().unwrap();
}