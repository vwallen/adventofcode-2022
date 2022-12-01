
use adventofcode_2022::read_input;
use itertools::Itertools;

pub fn prepare(file_name: &str) -> Vec<Vec<u32>> {
    let input = read_input(file_name);
    let mut elves = Vec::<Vec<u32>>::new();
    for chunk in input.split("\n\n") {
        elves.push(
            chunk
                .lines()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        );
    }
    elves
}

pub fn part_1(elves: &Vec<Vec<u32>>) -> Option<u32> {
    let elves_calories = elves
        .iter()
        .map(|e| e.iter().sum::<u32>())
        .max()?;
    Some(elves_calories)
}

pub fn part_2(elves: &Vec<Vec<u32>>) -> Option<u32> {
    let elves_calories = elves
        .iter()
        .map(|e| e.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum();
    Some(elves_calories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_elves() {
        let elves = prepare("day01-example.txt");
        assert_eq!(elves[0], vec![1000, 2000, 3000]);
        assert_eq!(elves[elves.len() - 1], vec![10000])
    }

    #[test]
    fn test_part_1() {
        let elves = prepare("day01-example.txt");
        assert_eq!(part_1(&elves), Some(24000))
    }

    #[test]
    fn test_part_2() {
        let elves = prepare("day01-example.txt");
        assert_eq!(part_2(&elves), Some(45000))
    }
}
