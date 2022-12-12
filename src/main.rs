#[macro_use] extern crate scan_rules;
use colored::Colorize;

pub mod day01;
pub mod day02;
pub mod day02b;
pub mod day03;
pub mod day04;
pub mod day04b;
pub mod day04c;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

fn main() {
    println!("{}", "=== Advent of Code 2022 ====".bright_red());

    println!("{} {} {}", "----------".red(), "Day 01".bright_green(), "----------".red());
    let elves_day1 = day01::prepare("day01.txt");
    println!("⭐ {}", day01::part_1(&elves_day1).unwrap());
    println!("⭐ {}", day01::part_2(&elves_day1).unwrap());

    println!("{} {} {}", "----------".red(), "Day 02".bright_green(), "----------".red());
    let input_day2 = day02::prepare("day02.txt");
    println!("⭐ {}", day02::part_1(&input_day2).unwrap());
    println!("⭐ {}", day02::part_2(&input_day2).unwrap());
    println!("{}", "---".green());
    let input_day2b = day02b::prepare("day02.txt");
    println!("⭐ {}", day02b::part_1(&input_day2b).unwrap());
    println!("⭐ {}", day02b::part_2(&input_day2b).unwrap());

    println!("{} {} {}", "----------".red(), "Day 03".bright_green(), "----------".red());
    let input_day3 = day03::prepare("day03.txt");
    println!("⭐ {}", day03::part_1(&input_day3).unwrap());
    println!("⭐ {}", day03::part_2(&input_day3).unwrap());

    println!("{} {} {}", "----------".red(), "Day 04".bright_green(), "----------".red());
    let input_day4 = day04::prepare("day04.txt");
    println!("⭐ {}", day04::part_1(&input_day4).unwrap());
    println!("⭐ {}", day04::part_2(&input_day4).unwrap());
    println!("{}", "---".green());
    let input_day4b = day04b::prepare("day04.txt");
    println!("⭐ {}", day04b::part_1(&input_day4b).unwrap());
    println!("⭐ {}", day04b::part_2(&input_day4b).unwrap());
    println!("{}", "---".green());
    if let Ok(input_day4c) = day04c::prepare("day04.txt") {
        println!("⭐ {}", day04c::part_1(&input_day4c).unwrap());
        println!("⭐ {}", day04c::part_2(&input_day4c).unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day 05".bright_green(), "----------".red());
    if let Ok((mut day05_inventory, day05_commands)) = day05::prepare("day05.txt") {
        println!("⭐ {}", day05::part_1(&mut day05_inventory, day05_commands).unwrap());
    }
    if let Ok((mut day05_inventory, day05_commands)) = day05::prepare("day05.txt") {
        println!("⭐ {}", day05::part_2(&mut day05_inventory, day05_commands).unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day 06".bright_green(), "----------".red());
    if let Ok(input_day06) = day06::prepare("day06.txt") {
        println!("⭐ {}", day06::part_1(&input_day06).unwrap());
        println!("⭐ {}", day06::part_2(&input_day06).unwrap());
    }

    println!("{} {} {}", "----------".red(), "Day 07".bright_green(), "----------".red());
    println!("💀");
    println!("💀");

    println!("{} {} {}", "----------".red(), "Day 08".bright_green(), "----------".red());
    println!("💀");
    println!("💀");

    println!("{} {} {}", "----------".red(), "Day 09".bright_green(), "----------".red());
    println!("💀");
    println!("💀");

    println!("{} {} {}", "----------".red(), "Day 10".bright_green(), "----------".red());
    if let Ok(day10_input) = day10::prepare("day10.txt") {
        println!("⭐ {}", day10::part_1(&day10_input).unwrap());
        println!("⭐");
        let pattern = day10::part_2(&day10_input).unwrap();
        day10::print_pattern(&pattern, 40);
    }

    println!("{} {} {}", "----------".red(), "Day 10".bright_green(), "----------".red());
    if let Ok(day11_input) = day11::prepare("day11.txt") {
        println!("⭐ {}", day11::part_1(day11_input).unwrap());
        println!("💀");
    }

    /*

    println!("---------- Day 12 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 13 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 14 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 15 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 16 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 17 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 18 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 19 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 20 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 21 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 22 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 23 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 24 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 25 ----------");
    println!("💀");
    println!("💀");
*/
    println!("{}", "============================".bright_red());
}
