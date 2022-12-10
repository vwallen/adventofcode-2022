#[macro_use] extern crate scan_rules;

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

fn main() {
    println!("=== Advent of Code 2022 ====");

    println!("---------- Day 01 ----------");
    let elves_day1 = day01::prepare("day01.txt");
    println!("⭐ {}", day01::part_1(&elves_day1).unwrap());
    println!("⭐ {}", day01::part_2(&elves_day1).unwrap());

    println!("---------- Day 02 ----------");
    let input_day2 = day02::prepare("day02.txt");
    println!("⭐ {}", day02::part_1(&input_day2).unwrap());
    println!("⭐ {}", day02::part_2(&input_day2).unwrap());
    println!("---");
    let input_day2b = day02b::prepare("day02.txt");
    println!("⭐ {}", day02b::part_1(&input_day2b).unwrap());
    println!("⭐ {}", day02b::part_2(&input_day2b).unwrap());

    println!("---------- Day 03 ----------");
    let input_day3 = day03::prepare("day03.txt");
    println!("⭐ {}", day03::part_1(&input_day3).unwrap());
    println!("⭐ {}", day03::part_2(&input_day3).unwrap());

    println!("---------- Day 04 ----------");
    let input_day4 = day04::prepare("day04.txt");
    println!("⭐ {}", day04::part_1(&input_day4).unwrap());
    println!("⭐ {}", day04::part_2(&input_day4).unwrap());
    println!("---");
    let input_day4b = day04b::prepare("day04.txt");
    println!("⭐ {}", day04b::part_1(&input_day4b).unwrap());
    println!("⭐ {}", day04b::part_2(&input_day4b).unwrap());
    println!("---");
    if let Ok(input_day4c) = day04c::prepare("day04.txt") {
        println!("⭐ {}", day04c::part_1(&input_day4c).unwrap());
        println!("⭐ {}", day04c::part_2(&input_day4c).unwrap());
    }

    println!("---------- Day 05 ----------");
    if let Ok((mut day05_inventory, day05_commands)) = day05::prepare("day05.txt") {
        println!("⭐ {}", day05::part_1(&mut day05_inventory, day05_commands).unwrap());
    }
    if let Ok((mut day05_inventory, day05_commands)) = day05::prepare("day05.txt") {
        println!("⭐ {}", day05::part_2(&mut day05_inventory, day05_commands).unwrap());
    }

    println!("---------- Day 06 ----------");
    if let Ok(input_day06) = day06::prepare("day06.txt") {
        println!("⭐ {}", day06::part_1(&input_day06).unwrap());
        println!("⭐ {}", day06::part_2(&input_day06).unwrap());
    }

    println!("---------- Day 07 ----------");
    println!("💀");
    println!("💀");

    println!("---------- Day 08 ----------");
    println!("💀");
    println!("💀");


}
