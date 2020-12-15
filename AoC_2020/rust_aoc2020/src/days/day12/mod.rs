use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day12/{}.txt", filename))
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

#[derive(Clone, Copy)]
enum Facing {
    N = 0,
    E = 90,
    S = 180,
    W = 270,
}

fn calculate_one(input: &str) -> Option<i32> {
    let mut latitude: i32 = 0;
    let mut longitude: i32 = 0;

    let mut facing = Facing::E;

    for line in input.lines() {
        let (operator, value) = line.split_at(1);

        match operator {
            "L" => {
                let facing_value: i32 = facing as i32;
                let new_facing = facing_value - to_int(value);
                facing = get_new_facing(new_facing).unwrap();
            }
            "R" => {
                let facing_value: i32 = facing as i32;
                let new_facing = facing_value + to_int(value);
                facing = get_new_facing(new_facing).unwrap();
            }
            "F" => {
                let (lat, long) = get_new_pos(latitude, longitude, facing, value);
                latitude = lat;
                longitude = long;
            }
            &_ => {
                let new_facing: Facing = match_facing_on_str(operator).unwrap();
                let (lat, long) = get_new_pos(latitude, longitude, new_facing, value);
                latitude = lat;
                longitude = long;
            }
        }
    }

    return Some(latitude + longitude);
}

fn get_new_facing(new_facing: i32) -> Option<Facing> {
    let new = (new_facing + 360) % 360;

    match new {
        x if x == Facing::N as i32 => Some(Facing::N),
        x if x == Facing::E as i32 => Some(Facing::E),
        x if x == Facing::S as i32 => Some(Facing::S),
        x if x == Facing::W as i32 => Some(Facing::W),
        _ => None,
    }
}

fn match_facing_on_str(facing_str: &str) -> Option<Facing> {
    match facing_str {
        "N" => Some(Facing::N),
        "E" => Some(Facing::E),
        "S" => Some(Facing::S),
        "W" => Some(Facing::W),
        _ => None,
    }
}

fn get_new_pos(latitude: i32, longitude: i32, facing: Facing, value: &str) -> (i32, i32) {
    let mut lat = latitude;
    let mut long = longitude;

    match facing {
        Facing::N => lat -= to_int(value),
        Facing::E => long += to_int(value),
        Facing::S => lat += to_int(value),
        Facing::W => long -= to_int(value),
    }

    (lat, long)
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<i32> {
    let mut ship_latitude: i32 = 0;
    let mut ship_longitude: i32 = 0;
    let mut waypoint_rel_latitude: i32 = -1;
    let mut waypoint_rel_longitude: i32 = 10;

    for line in input.lines() {
        let (operator, value) = line.split_at(1);

        match operator {
            "L" => {
                let (new_waypoint_lat, new_waypoint_long) =
                    rotate_right(waypoint_rel_latitude, waypoint_rel_longitude, value, -1).unwrap();
                waypoint_rel_latitude = new_waypoint_lat;
                waypoint_rel_longitude = new_waypoint_long;
            }
            "R" => {
                let (new_waypoint_lat, new_waypoint_long) =
                    rotate_right(waypoint_rel_latitude, waypoint_rel_longitude, value, 1).unwrap();
                waypoint_rel_latitude = new_waypoint_lat;
                waypoint_rel_longitude = new_waypoint_long;
            }
            "F" => {
                ship_latitude += waypoint_rel_latitude * to_int(value);
                ship_longitude += waypoint_rel_longitude * to_int(value);
            }
            &_ => {
                let new_facing: Facing = match_facing_on_str(operator).unwrap();
                let (lat, long) = get_new_pos(
                    waypoint_rel_latitude,
                    waypoint_rel_longitude,
                    new_facing,
                    value,
                );
                waypoint_rel_latitude = lat;
                waypoint_rel_longitude = long;
            }
        }
    }
    return Some(ship_latitude.abs() + ship_longitude.abs());
}

fn rotate_right(
    waypoint_lat: i32,
    waypoint_long: i32,
    value: &str,
    modifier: i32,
) -> Option<(i32, i32)> {
    match value {
        "90" => Some((waypoint_long * modifier, -waypoint_lat * modifier)),
        "270" => Some((-waypoint_long * modifier, waypoint_lat * modifier)),
        "180" => Some((-waypoint_lat, -waypoint_long)),
        &_ => None,
    }
}

fn to_int(string: &str) -> i32 {
    return string.parse::<i32>().unwrap();
}
