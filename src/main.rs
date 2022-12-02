
pub mod day01;
pub mod day02;

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
}
