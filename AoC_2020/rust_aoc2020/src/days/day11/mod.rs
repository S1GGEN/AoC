use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day11/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input: &str) -> String {
    let result = calculate_one(input);

    if result == None {
        return format!("Task 1: FAILED");
    }
    return format!("Task 1: {}", result.unwrap());
}

fn calculate_one(input: &str) -> Option<u32> {
    let mut last_iteration = seat_iteration(input);

    loop {
        let this_iteration = seat_iteration(&*last_iteration);
        if last_iteration == this_iteration {
            // This is probably immensely slow
            break;
        }
        last_iteration = this_iteration;
    }

    let count_occupied = last_iteration.matches('#').count();

    return Some(count_occupied as u32);
}

fn seat_iteration(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let seating_width = lines[0].len();

    let mut new_seat_chart = "".to_owned();

    for y in 0..lines.len() {
        // println!("{}", lines[y]);
        let mut new_line = "".to_owned();
        for x in 0..seating_width {
            let seat = lines[y].chars().nth(x).unwrap();
            if seat != '.' {
                let num_adjacent =
                    num_adjacent_seats_occupied(&lines, seating_width as u32, (x as i32, y as i32));
                if seat == 'L' && num_adjacent == 0 {
                    new_line.push('#')
                } else if seat == '#' && num_adjacent >= 4 {
                    new_line.push('L')
                } else {
                    new_line.push(seat)
                }
            } else {
                new_line.push(seat)
            }
        }
        new_line.push('\n');

        new_seat_chart.push_str(&*new_line);
    }

    // println!("\n\n{}", new_seat_chart);

    return new_seat_chart;
}

fn num_adjacent_seats_occupied(lines: &Vec<&str>, seating_width: u32, pos: (i32, i32)) -> u32 {
    let (x, y) = pos;
    let mut num_occupied = 0;

    // NORTH
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x, y - 1)) as u32;
    // NORTH-WEST
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x + 1, y - 1)) as u32;
    // WEST
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x + 1, y)) as u32;
    // SOUTH-WEST
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x + 1, y + 1)) as u32;
    // SOUTH
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x, y + 1)) as u32;
    // SOUTH-EAST
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x - 1, y + 1)) as u32;
    // EAST
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x - 1, y)) as u32;
    // NORTH-EAST
    num_occupied += safe_seat_is_occupied(&lines, seating_width, (x - 1, y - 1)) as u32;

    return num_occupied;
}

fn safe_seat_is_occupied(lines: &Vec<&str>, seating_width: u32, pos: (i32, i32)) -> bool {
    let (x, y) = pos;
    // Check if on board
    if x >= 0 && x < seating_width as i32 && y >= 0 && y < lines.len() as i32 {
        if lines[y as usize].chars().nth(x as usize) == Some('#') {
            return true;
        }
    }
    // If not on board, return true, walls count as unoccupied
    return false;
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    if result == None {
        return format!("Task 2: FAILED");
    }
    return format!("Task 2: {}", result.unwrap());
}

fn calculate_two(input: &str) -> Option<u32> {
    // println!("\n\n{}", input);
    let mut last_iteration = seat_iteration_2(input);

    loop {
        let this_iteration = seat_iteration_2(&*last_iteration);
        if last_iteration == this_iteration {
            break;
        }
        last_iteration = this_iteration;
    }

    let count_occupied = last_iteration.matches('#').count();

    return Some(count_occupied as u32);
}

fn seat_iteration_2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let seating_width = lines[0].len();

    let mut new_seat_chart = "".to_owned();

    for y in 0..lines.len() {
        // println!("{}", lines[y]);
        let mut new_line = "".to_owned();
        for x in 0..seating_width {
            let seat = lines[y].chars().nth(x).unwrap();
            if seat != '.' {
                let num_in_sight =
                    num_in_sight_seats_occupied(&lines, seating_width as u32, (x as i32, y as i32));
                if seat == 'L' && num_in_sight == 0 {
                    new_line.push('#')
                } else if seat == '#' && num_in_sight >= 5 {
                    new_line.push('L')
                } else {
                    new_line.push(seat)
                }
            } else {
                new_line.push(seat)
            }
        }
        new_line.push('\n');

        new_seat_chart.push_str(&*new_line);
    }

    // println!("\n\n{}", new_seat_chart);

    return new_seat_chart;
}

fn num_in_sight_seats_occupied(lines: &Vec<&str>, seating_width: u32, pos: (i32, i32)) -> u32 {
    let (x, y) = pos;
    let mut num_occupied = 0;

    // NORTH
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (0, -1)) as u32;
    // NORTH-WEST
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (1, -1)) as u32;
    // WEST
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (1, 0)) as u32;
    // SOUTH-WEST
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (1, 1)) as u32;
    // SOUTH
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (0, 1)) as u32;
    // SOUTH-EAST
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (-1, 1)) as u32;
    // EAST
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (-1, 0)) as u32;
    // NORTH-EAST
    num_occupied += check_dir_occupied(&lines, seating_width, (x, y), (-1, -1)) as u32;

    return num_occupied;
}

fn check_dir_occupied(
    lines: &Vec<&str>,
    seating_width: u32,
    pos: (i32, i32),
    dir: (i32, i32),
) -> bool {
    let (mut x, mut y) = pos;
    let (x_dir, y_dir) = dir;

    while x + x_dir >= 0
        && x + x_dir < seating_width as i32
        && y + y_dir >= 0
        && y + y_dir < lines.len() as i32
    {
        x += x_dir;
        y += y_dir;

        if lines[y as usize].chars().nth(x as usize) == Some('#') {
            return true;
        } else if lines[y as usize].chars().nth(x as usize) == Some('L') {
            return false;
        }
    }

    // If occupied found,  the coast is clear
    return false;
}
