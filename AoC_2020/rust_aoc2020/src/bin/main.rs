use rust_aoc2020::days::{day01, day02, day03, day04, day05, day06, day08, day09};

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


    let input_4 = day04::load_input("input");
    println!("Day 4 {}", day04::one(&input_4));
    println!("Day 4 {}\n", day04::two(&input_4));

    let input_5 = day05::load_input("input");
    println!("Day 5 {}\n", day05::both(&input_5));

    let input_6 = day06::load_input("input");
    println!("Day 6 {}", day06::one(&input_6));
    println!("Day 6 {}\n", day06::two(&input_6));

    /*
    // Day 7 ragequit
    let input_7 = day07::load_input("input");
    println!("Day 6 {}", day07::one(&input_7));
    println!("Day 6 {}\n", day07::two(&input_7));
     */


    let input_8 = day08::load_input("input");
    println!("Day 8 {}", day08::one(&input_8));
    println!("Day 8 {}\n", day08::two(&input_8));

    let input_9 = day09::load_input("input");
    println!("Day 9 {}", day09::one(&input_9));
    println!("Day 9 {}\n", day09::two(&input_9, None));
}
