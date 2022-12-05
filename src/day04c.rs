use adventofcode_2022::read_input_lines;
use std::str::FromStr;
use anyhow::{Context, Result};

pub struct Interval { start:u16, end:u16 }
impl Interval {
    fn is_overlapping(&self, other: &Interval) -> bool {
        (self.start >= other.start && self.end <= other.end) ||
        (other.start >= self.start && other.end <= self.end)
    }    
    fn is_disjoint(&self, other: &Interval) -> bool {
        self.end < other.start || other.end < self.start
    }    
}
impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, e) = s.split_once('-').context(format!("Unable to parse interval: {}", s))?;
        Ok(Self{ start: s.parse()?, end: e.parse()? })
    }
}

pub struct Assignment { a:Interval, b:Interval }
impl Assignment {
    fn is_overlapping(&self) -> bool { self.a.is_overlapping(&self.b) }    
    fn is_disjoint(&self) -> bool { self.a.is_disjoint(&self.b) }    
}
impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').context(format!("Unable to parse assignment: {}", s))?;
        Ok(Self{ a: a.parse()?, b: b.parse()? })
    }
}

pub fn prepare(file_name:&str) -> Result<Vec<Assignment>> {
    read_input_lines(file_name)
        .iter()
        .map(|line| line.parse())
        .collect()
}

pub fn part_1(input: &Vec<Assignment>) -> Option<u32> {
    let count = input
        .iter()
        .filter(|a| a.is_overlapping())
        .count();
    Some(count as u32)
}

pub fn part_2(input: &Vec<Assignment>) -> Option<u32> {
    let count = input
        .iter()
        .filter(|a| !a.is_disjoint())
        .count();
    Some(count as u32)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day04-example.txt").unwrap();
        assert!(input[3].is_overlapping());
        assert_eq!(input[3].is_disjoint(), false);
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day04-example.txt").unwrap();
        assert_eq!(part_1(&input), Some(2))
    }

    #[test]
    fn test_part_2() {
        let input = prepare("day04-example.txt").unwrap();
        assert_eq!(part_2(&input), Some(4))
    }


}