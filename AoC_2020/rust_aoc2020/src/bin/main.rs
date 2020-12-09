use rust_aoc2020::days::{day1, day2, day3, day4, day5, day6, day8, day9};

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
    println!("Day 5 {}\n", day5::both(&input_5));

    let input_6 = day6::load_input("input");
    println!("Day 6 {}", day6::one(&input_6));
    println!("Day 6 {}\n", day6::two(&input_6));

    /*
    // Day 7 ragequit
    let input_7 = day7::load_input("input");
    println!("Day 6 {}", day7::one(&input_7));
    println!("Day 6 {}\n", day7::two(&input_7));
     */


    let input_8 = day8::load_input("input");
    println!("Day 8 {}", day8::one(&input_8));
    println!("Day 8 {}\n", day8::two(&input_8));

    let input_9 = day9::load_input("input");
    println!("Day 9 {}", day9::one(&input_9));
    println!("Day 9 {}\n", day9::two(&input_9, None));
}
