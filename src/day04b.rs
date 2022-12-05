use adventofcode_2022::read_input_lines;
use std::collections::HashSet;

fn parse_range(pattern: &str) -> HashSet<u16> {
    let mut range = HashSet::<u16>::new();
    if let Some((a, b)) = pattern.split_once('-') {
        (a.parse().unwrap()..=b.parse().unwrap()).for_each(|n| { range.insert(n); })
    }
    range    
}

pub fn prepare(file_name:&str) -> Vec<(HashSet<u16>, HashSet<u16>)> {
    let mut input = Vec::new();
    for line in read_input_lines(file_name).iter() {
        if let Some((a, b)) = line.split_once(',') {
            input.push((parse_range(a), parse_range(b)));
        }
    }
    input
}

pub fn part_1(input: &[(HashSet<u16>, HashSet<u16>)]) -> Option<u32> {
    let count = input
        .iter()
        .filter(|(a, b)| (a & b).len() == a.len() || (a & b).len() == b.len())
        .count();
    Some(count as u32)
}

pub fn part_2(input: &[(HashSet<u16>, HashSet<u16>)]) -> Option<u32> {
    let count = input
        .iter()
        .filter(|(a, b)| !(a & b).is_empty())
        .count();
    Some(count as u32)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day04-example.txt");
        assert!(input[0].0.contains(&2));
        assert!(input[0].1.contains(&8));
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