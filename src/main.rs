
pub mod day01;

fn main() {
    println!("=== Advent of Code 2022 ====");

    println!("---------- Day 01 ----------");
    let elves_day1 = day01::prepare("day01.txt");
    println!("⭐ {}", day01::part_1(&elves_day1).unwrap());
    println!("⭐ {}", day01::part_2(&elves_day1).unwrap());
}
