use adventofcode_2022::read_input_lines;
use std::collections::HashSet;

const CODES:&str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn prepare(file_name: &str) -> Vec<String> {
    read_input_lines(file_name)
        .iter()
        .map(| line | String::from(line))
        .collect::<Vec<String>>()
}

pub fn part_1(input: &Vec<String>) -> Option<u32> {
    let mut items:Vec<char> = Vec::new();
    for pair in input.iter().map(|x| x.split_at(x.len()/2)) {
        let a: HashSet<char> = pair.0.chars().into_iter().collect();
        let b: HashSet<char> = pair.1.chars().into_iter().collect();
        let c: &char = a.intersection(&b).collect::<HashSet<&char>>().iter().next().unwrap();
        items.push(*c);
    }
    let result:usize = items.iter().map(|c| CODES.find(*c).unwrap()).sum();
    Some(result as u32)
}

pub fn part_2(input: &Vec<String>) -> Option<u32> {
    let mut items:Vec<char> = Vec::new();
    for sack in input.chunks(3) {
        let a: HashSet<char> = sack[0].chars().into_iter().collect();
        let b: HashSet<char> = sack[1].chars().into_iter().collect();
        let c: HashSet<char> = sack[2].chars().into_iter().collect();
        let ab: HashSet<&char> = a.intersection(&b).collect();
        let bc: HashSet<&char> = b.intersection(&c).collect();
        let abc: HashSet<&&char> = ab.intersection(&bc).collect();
        let ch: &&char = abc.iter().next().unwrap();
        items.push(**ch);
    }
    let result:usize = items.iter().map(|c| CODES.find(*c).unwrap()).sum();
    Some(result as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day03-example.txt");
        assert_eq!(input[0], "vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(input[4],  "ttgJtRGJQctTZtZT".to_string());
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day03-example.txt");
        assert_eq!(part_1(&input), Some(157))
    }

    #[test]
    fn test_part_2() {
        let input = prepare("day03-example.txt");
        assert_eq!(part_2(&input), Some(70))
    }
}