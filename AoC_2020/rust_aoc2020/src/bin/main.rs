use rust_aoc2020::days::{day1, day2, day3};

fn main() {
    println!("Hello, world!");
    /*
    let mut input_1 = day1::parser::parse("input");
    println!("{}", day1::one(&mut input_1));
    println!("{}", day1::two(&mut input_1));
    let input_2 = day2::parser::parse("input");
    println!("{}", day2::one(&input_2));
    println!("{}", day2::two(&input_2));
     */
    let input_3 = day3::load_input("input");
    println!("{}", day3::one(&input_3));
    println!("{}", day3::two(&input_3));
}
