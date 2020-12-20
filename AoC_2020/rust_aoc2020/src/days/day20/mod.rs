use std::fs;
use std::str;
use std::collections::HashMap;
use std::ops::Index;
use conditional::conditional;
use std::iter::Enumerate;
use std::slice::Iter;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day20/{}.txt", filename))
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

fn get_directions() -> Enumerate<Iter<'static, &'static str>> {
    ["top", "right", "bottom", "left"].iter().enumerate()
}

fn calculate_one(input: &str) -> Option<u64> {
    let tiles_map = get_tiles_parsed(input);
    let mut tile_matches: HashMap<u32, TileMatch> = HashMap::new();

    for (id_1, tile_1) in &tiles_map {
        let mut id_1_matches = vec![None; 4];
        for (id_2, tile_2) in &tiles_map {
            let mut id_2_matches = vec![None; 4];
            if id_1 != id_2 {
                for (dir_index_1, dir_1) in get_directions() {
                    for (dir_index_2, dir_2) in get_directions() {
                        let my_tile2_dir_2: Vec<bool> = tile_2[dir_2].clone().into_iter().rev().collect();
                        if tile_1[dir_1] == tile_2[dir_2] || tile_1[dir_1] == my_tile2_dir_2 {
                            id_1_matches[dir_index_1] = Some(*id_2);
                            id_2_matches[dir_index_2] = Some(*id_1);
                        }
                    }
                }
            }
            tile_matches.insert(*id_2, get_new_tilematches(&id_2_matches, tile_matches.get(id_2)));
        }
        tile_matches.insert(*id_1, get_new_tilematches(&id_1_matches, tile_matches.get(id_1)));
    }

    // println!("{:?}", get_corners(&tile_matches));

    let corners = get_corners(&tile_matches);
    let mut result: u64 = 1;

    for corner in corners {
        result *= corner as u64
    }


    Some(result)
}

fn get_corners(tile_matches : &HashMap<u32, TileMatch>) -> Vec<u32> {
    let mut corners : Vec<u32> = vec![];
    for (id, tile_match) in tile_matches {
        let mut count = 0;
        for (_, dir) in get_directions() {
            if  tile_match[dir].is_some() {
                count += 1;
            }
        }
        if count == 2 {
            corners.push(*id);
        }
    }
    corners
}


#[derive(Debug)]
pub struct Tile {
    top: Vec<bool>,
    right: Vec<bool>,
    bottom: Vec<bool>,
    left: Vec<bool>
}

impl Index<&'_ str> for Tile {
    type Output = Vec<bool>;
    fn index(&self, s: &str) -> &Vec<bool> {
        match s {
            "top" => &self.top,
            "right" => &self.right,
            "bottom" => &self.bottom,
            "left" => &self.left,
            _ => panic!("unknown field: {}", s),
        }
    }
}

#[derive(Debug)]
pub struct TileMatch {
    top: Option<u32>,
    right: Option<u32>,
    bottom: Option<u32>,
    left: Option<u32>
}


impl Index<&'_ str> for TileMatch {
    type Output =  Option<u32>;
    fn index(&self, s: &str) -> & Option<u32> {
        match s {
            "top" => &self.top,
            "right" => &self.right,
            "bottom" => &self.bottom,
            "left" => &self.left,
            _ => panic!("unknown field: {}", s),
        }
    }
}

/*
fn get_tilematches(matches : &Vec<Option<u32>>) -> TileMatch {
    TileMatch {
        top: matches[0],
        right: matches[1],
        bottom: matches[2],
        left: matches[3]
    }
}
 */


fn get_new_tilematches(matches : &Vec<Option<u32>>, tilematches : Option<&TileMatch>) -> TileMatch {
    match tilematches {
        None => {
            TileMatch {
                top: matches[0],
                right: matches[1],
                bottom: matches[2],
                left: matches[3],
            }
        }
        _ => {
            let tilematches_unwrapped = tilematches.unwrap();
            TileMatch {
                top: conditional!(tilematches_unwrapped.top.is_some() ? tilematches_unwrapped.top : matches[0]),
                right: conditional!(tilematches_unwrapped.right.is_some() ? tilematches_unwrapped.right : matches[1]),
                bottom: conditional!(tilematches_unwrapped.bottom.is_some() ? tilematches_unwrapped.bottom : matches[2]),
                left: conditional!(tilematches_unwrapped.left.is_some() ? tilematches_unwrapped.left : matches[3])
            }
        }
    }

}

/*
fn bool_to_char(bool : bool) -> char {
    return match bool {
        true => '#',
        false => '.'
    }
}
 */

fn char_to_bool(char : char) -> Option<bool> {
    return match char {
        '#' => Some(true),
        '.' => Some(false),
        _ => None
    }
}

fn line_to_bools(line : &str) -> Vec<bool> {
    let mut bool_line = vec![];
    for char in line.chars() {
        bool_line.push(char_to_bool(char).unwrap());
    }

    bool_line
}

fn get_tiles_parsed(input : &str) -> HashMap<u32, Tile> {
    let tiles = input.split("\n\n");

    let mut tiles_parsed : HashMap<u32, Tile> = HashMap::new();

    for raw_tile in tiles {
        let tile_lines : Vec<&str> = raw_tile.lines().collect();
        let first_line = tile_lines[0].replace(":", "");
        let (_, id) = first_line.split_at(5);

        let top = line_to_bools(tile_lines[1]);
        let mut right = vec![];
        let bottom = line_to_bools(tile_lines[tile_lines.len() - 1]);
        let mut left = vec![];

        for i in 1..tile_lines.len() {
            let line_chars: Vec<char> = tile_lines[i].chars().collect();
            right.push(char_to_bool(line_chars[line_chars.len() - 1]).unwrap());
            left.push(char_to_bool(line_chars[0]).unwrap());
        }

        let tile = Tile {
            top,
            right,
            bottom,
            left
        };
        tiles_parsed.insert(to_int(id), tile);
    }

    tiles_parsed
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<u64> {
    None
}

fn to_int(string: &str) -> u32 {
    return string.parse::<u32>().unwrap();
}