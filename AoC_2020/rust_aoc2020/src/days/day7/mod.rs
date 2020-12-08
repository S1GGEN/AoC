/**
 * Had to rage quit and do this in python instead
*/

use std::fs;
use std::str;
use std::collections::{HashSet, HashMap};

pub fn load_input(filename : &str) -> String{
    let input = fs::read_to_string(format!("src/days/day7/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input : &str) -> String {
    // DAGGY
    // https://github.com/mitchmindtree/daggy

    for line in input.lines() {
        parse_line(line);
    }
    return format!("Task 1: {}", "aa");
}

pub fn two(input : &str) -> String {
    return format!("Task 2: {}", "aaa");
}

fn parse_line(line : &str) {
    /*
    let mut words: Vec<&str> = line.split(" ")
        .filter(
            |x| (
                x != &"contain"
                && !x.contains("bag")
                && x.len() != 1
            )
        ).collect();
    println!("{:?}", words);
    let this_bag_string = [words[0], words[1]].join(" ");
    // let this_bag_string = this_bag.join(" ");
    println!("{}", this_bag_string);

    let mut sub_bags : Vec<&str> = vec![];
    for i in (2..words.len()).step_by(2) {
        let mut my_string = "".to_owned();
        my_string.push_str(&words[i]);
        my_string.push_str(&words[i+1]);
            /*
            let next_bag = [words[i].to_owned(), words[i+1].to_owned()].join(" ");
            // let next_bag_string = (next_bag as String).clone() as &str;

             */
        let mut next_bag_vec: Vec<&str> = vec![&my_string];
        println!("sub bag: {:?}", next_bag_vec);
        sub_bags.append(&mut next_bag_vec);
    }
    println!("{:?}", &sub_bags);

    if words[3] != "no" {
        println!("not empty!");

    }

     */
}
