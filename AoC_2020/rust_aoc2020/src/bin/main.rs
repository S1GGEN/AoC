use rust_aoc2020::days::{day1, day2, day3};

fn main() {
    println!("Hello, world!");
    let input_1 = day1::load_input("input");
    println!("Day 1 {}", day1::one(&input_1));
    println!("Day 1 {}", day1::two(&input_1));


    let input_2 = day2::load_input("input");
    println!("Day 2 {}", day2::one(&input_2));
    println!("Day 2 {}", day2::two(&input_2));

    let input_3 = day3::load_input("input");
    println!("Day 3 {}", day3::one(&input_3));
    println!("Day 3 {}", day3::two(&input_3));
}
