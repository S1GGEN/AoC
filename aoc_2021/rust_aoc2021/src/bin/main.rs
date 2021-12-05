use rust_aoc2021::days::{
    day01, day02, day03
};

fn main() {
    let input_1 = day01::load_input("input");
    println!("Day 1 {}", day01::one(&input_1));
    println!("Day 1 {}\n", day01::two(&input_1));

    let input_2 = day02::load_input("input");
    println!("Day 2 {}", day02::one(&input_2));
    println!("Day 2 {}\n", day02::two(&input_2));


    let input_3 = day03::load_input("input");
    println!("Day 3 {}", day03::one(&input_3));
    println!("Day 3 {}\n", day03::two(&input_3));
}
