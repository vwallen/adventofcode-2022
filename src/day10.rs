
use std::collections::VecDeque;
use colored::Colorize;
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

pub fn calculate_signal(instructions: &[OpCode]) -> Result<VecDeque<isize>> {
    let mut signals:VecDeque<isize> = VecDeque::new();
    for op_code in instructions.iter() {
        match op_code {
            OpCode::NoOp => { signals.push_back(0); },
            OpCode::AddX(x) => { 
                signals.push_back(0);
                signals.push_back(*x);
            },
        }
    }
    Ok(signals)
}

pub fn print_pattern(pattern: &[char], w:usize) {
    for (i, ch) in pattern.iter().enumerate() {
        let st = if ch == &'.' { String::from(*ch).red() } else { String::from(*ch).bright_green() };
        if (i + 1) % w == 0 {
                println!("{}", st);
        } else {
                print!("{}", st);
        }
    }
}

pub fn part_1(instructions: &[OpCode]) -> Option<isize> {
    if let Ok(signals) = calculate_signal(instructions) {
        let mut signal_strengths = VecDeque::new();
        let mut x_register:isize = 1;
        for (c, signal) in signals.iter().enumerate() {
            let cycle:isize = c as isize + 1;
            match cycle {
                20|60|100|140|180|220 => { 
                    signal_strengths.push_front(cycle * x_register);
                },
                _ => { }
            }
            x_register += signal;
        }
        let signal_strength = signal_strengths.iter().sum();
        Some(signal_strength)
    } else { None }
}


pub fn part_2(instructions: &[OpCode]) -> Option<Vec<char>> {
    if let Ok(signals) = calculate_signal(instructions) {
        let mut pattern = vec!['.';240];
        let mut x_register:isize = 1;
        for (cycle, signal) in signals.iter().enumerate() {
            let beam_col = cycle % 40;
            let pixl_col = x_register % 40;
            if (beam_col as isize - pixl_col).abs() < 2 {
                pattern.splice(cycle..=cycle, ['#']);
            }
            x_register += signal;
        }
        Some(pattern)
    } else { None }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
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
    fn test_part_2() {
        if let Ok(input) = prepare("day10-example.txt") {
            let test_pattern:Vec<char> = String::from("##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....").chars().collect();
            let elfs_pattern:Vec<char> = part_2(&input).unwrap();
            println!("----------------------------------------");
            print_pattern(&test_pattern, 40);
            println!("----------------------------------------");
            print_pattern(&elfs_pattern, 40);
            println!("----------------------------------------");
            assert_eq!(elfs_pattern, test_pattern);
        }
    }
}