use rust_aoc2020::days::{day1, day2, day3, day4, day5};

fn main() {
    let input_1 = day1::load_input("input");
    println!("Day 1 {}", day1::one(&input_1));
    println!("Day 1 {}\n", day1::two(&input_1));

    let input_2 = day2::load_input("input");
    println!("Day 2 {}", day2::one(&input_2));
    println!("Day 2 {}\n", day2::two(&input_2));


    let input_3 = day3::load_input("input");
    println!("Day 3 {}", day3::one(&input_3));
    println!("Day 3 {}\n", day3::two(&input_3));


    let input_4 = day4::load_input("input");
    println!("Day 4 {}", day4::one(&input_4));
    println!("Day 4 {}\n", day4::two(&input_4));

    let input_5 = day5::load_input("input");
    println!("Day 5 {}", day5::one(&input_5));
    println!("Day 5 {}\n", day5::two(&input_5));

}
