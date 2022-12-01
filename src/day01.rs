
use adventofcode_2022::read_input;

fn prepare(file_name: &str) -> Vec<Vec<u32>> {
    let input = read_input(file_name);
    let chunks = input.split("\n\n");
    let mut elves = Vec::<Vec<u32>>::new();
    for chunk in chunks {
        elves.push(chunk.lines().map(|n| n.parse().unwrap()).collect::<Vec<u32>>());
    }
    elves
}

pub fn part_1(file_name: &str) -> Option<u32> {
    let elves = prepare(file_name);
    let elves_calories = elves.iter().map(|e| e.iter().sum::<u32>()).collect::<Vec<u32>>();
    elves_calories.iter().max().copied()
}

pub fn part_2(file_name: &str) -> Option<u32> {
    let elves = prepare(file_name);
    let mut elves_calories = elves.iter().map(|e| e.iter().sum::<u32>()).collect::<Vec<u32>>();
    elves_calories.sort();
    elves_calories.reverse();
    elves_calories.truncate(3);
    Some(elves_calories.iter().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("day01-example.txt"), Some(24000))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("day01-example.txt"), Some(45000))
    }
}
