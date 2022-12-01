
pub mod day01;

fn main() {
    println!("=== Advent of Code 2022 ====");

    println!("---------- Day 01 ----------");
    println!("⭐ {}", day01::part_1("day01.txt").unwrap());
    println!("⭐ {}", day01::part_2("day01.txt").unwrap());
}
