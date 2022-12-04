use adventofcode_2022::read_input_lines;
use std::ops::RangeInclusive;

fn parse_range(pattern: &str) -> RangeInclusive<u16> {
    let mut rng = 0..=0;
    if let Some((a, b)) = pattern.split_once("-") {
        rng = RangeInclusive::new(a.parse().unwrap(), b.parse().unwrap())
    }
    rng    
}

pub fn prepare(file_name:&str) -> Vec<(RangeInclusive<u16>, RangeInclusive<u16>)> {
    let mut input = Vec::new();
    for line in read_input_lines(file_name).iter() {
        if let Some((a, b)) = line.split_once(",") {
            input.push((parse_range(a), parse_range(b)));
        }
    }
    input
}

pub fn part_1(input: &Vec<(RangeInclusive<u16>, RangeInclusive<u16>)>) -> Option<u32> {
    let count = input
        .iter()
        .filter(|(a, b)| (a.start() >= b.start() && a.end() <= b.end()) || (b.start() >= a.start() && b.end() <= a.end()))
        .count();
    Some(count as u32)
}

pub fn part_2(input: &Vec<(RangeInclusive<u16>, RangeInclusive<u16>)>) -> Option<u32> {
    let count = input
        .iter()
        .filter(|(a, b)| (a.end() >= b.start() && a.end() <= b.end()) || (b.end() >= a.start() && b.end() <= a.end()))
        .count();
    Some(count as u32)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day04-example.txt");
        assert_eq!(input[0].0, 2..=4);
        assert_eq!(input[0].1, 6..=8)
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day04-example.txt");
        assert_eq!(part_1(&input), Some(2))
    }

    #[test]
    fn test_part_2() {
        let input = prepare("day04-example.txt");
        assert_eq!(part_2(&input), Some(4))
    }


}