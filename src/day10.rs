
use std::collections::{VecDeque, vec_deque};

use adventofcode_2022::read_input;
use anyhow::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum OpCode {
    AddX(isize),
    NoOp,
}

pub fn prepare(file_name: &str) -> Result<Vec<OpCode>> {
    let input = read_input(file_name);
    let mut instructions = Vec::new();
    for line in input.lines() {
        let op_code = scan! { line;
            ("noop") => OpCode::NoOp,
            ("addx ", let x) => OpCode::AddX(x),
        };
        instructions.push(op_code.unwrap());
    }
    Ok(instructions)
}

pub fn part_1(instructions: &[OpCode]) -> Option<isize> {
    let mut signal_strengths = VecDeque::new();
    let mut x_register:isize = 1;
    let mut cycles:VecDeque<isize> = VecDeque::new();

    for op_code in instructions.iter() {
        match op_code {
            OpCode::NoOp => { cycles.push_back(0); },
            OpCode::AddX(x) => { 
                cycles.push_back(0);
                cycles.push_back(*x);
            },
        }
    }
    for cycle in 1..=220 {
        match cycle {
            20|60|100|140|180|220 => { 
                signal_strengths.push_front(cycle * x_register);
            },
            _ => { }
        }
        x_register += cycles.pop_front().unwrap();
    }
    let signal_strength = signal_strengths.iter().sum();
    Some(signal_strength)
}

pub fn part_2(_input: &[OpCode]) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn test_prepare() {
        // 0: addx 15
        // 1: addx -11
        // 8: noop
        if let Ok(instructions) = prepare("day10-example.txt") {
            assert_eq!(instructions.get(0).unwrap(), &OpCode::AddX(15));
            assert_eq!(instructions.get(1).unwrap(), &OpCode::AddX(-11));
            assert_eq!(instructions.get(9).unwrap(), &OpCode::NoOp);
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day10-example.txt") {
            assert_eq!(part_1(&input), Some(13140))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day10-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}