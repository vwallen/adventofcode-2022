use adventofcode_2022::read_input_lines;
use std::str::FromStr;
use std::num::ParseIntError;

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
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, e) = s.split_once("-").unwrap();
        Ok(Self{ start: s.parse()?, end: e.parse()? })
    }
}

pub struct Assignment { a:Interval, b:Interval }
impl Assignment {
    fn is_overlapping(&self) -> bool { self.a.is_overlapping(&self.b) }    
    fn is_disjoint(&self) -> bool { self.a.is_disjoint(&self.b) }    
}
impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(",").unwrap();
        Ok(Self{ a: a.parse()?, b: b.parse()? })
    }
}

pub fn prepare(file_name:&str) -> Vec<Assignment> {
    read_input_lines(file_name)
        .iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part_1(input: &Vec<Assignment>) -> Option<u32> {
    let count = input
        .iter()
        .fold(0, |n, a| if a.is_overlapping() { n+1 } else { n });
    Some(count as u32)
}

pub fn part_2(input: &Vec<Assignment>) -> Option<u32> {
    let count = input
        .iter()
        .fold(0, |n, a| if a.is_disjoint() { n } else { n+1 });
    Some(count as u32)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day04-example.txt");
        assert_eq!(input[3].is_overlapping(), true);
        assert_eq!(input[3].is_disjoint(), false);
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