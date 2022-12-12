
use adventofcode_2022::read_input;
use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum MonkeyOp {
    Adder(usize),
    Multer(usize),
    Square,
    Decider(usize, usize, usize)
}
impl MonkeyOp {
    pub fn call(&self, a:usize) -> usize {
        match self {
            Self::Adder(x)  => x + a,
            Self::Multer(x) => x * a,
            Self::Square    => a * a,
            Self::Decider(x, y, z) => if a % x == 0 { *y } else { *z }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Monkey {
    items: VecDeque<usize>,
    inspect: MonkeyOp,
    decider: MonkeyOp,
    activity: usize,
}
impl Monkey {
    pub fn inspect(&mut self) -> Option<usize> {
        if let Some(item) = self.items.front() {
            let val = self.inspect.call(*item);
            self.items[0] = val;
            self.activity += 1;
            Some(val)
        } else { None }
    }

    pub fn ennui(&mut self, ennui:usize, magic:usize) {
        if let Some(item) = self.items.front() {
            // I can't take credit for the magic number bit
            // and though I would have gotten there eventually
            // it would not have been without frustration
            self.items[0] = (item / ennui) % magic;
        }
    }

    pub fn decide(&mut self) -> Option<usize> {
        self.items.front().map(|item| self.decider.call(*item))
    }

    pub fn throw(&mut self) -> Option<usize> {
        self.items.pop_front()
    }

    pub fn catch(&mut self, item:usize) {
        self.items.push_back(item);
    }
}
impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let lines:Vec<&str> = s.split('\n').collect();
        // Monkey #:
        // Starting items: #, #, ...
        let_scan!(lines[1]; ("Starting items: ", [ let items: usize ], *: VecDeque<_>));

        // "Operation: new = old *|+ #|old"
        use MonkeyOp::*;
        let (op1, op2) = lines[2].strip_prefix("  Operation: new = old ").unwrap().split(' ').tuples().next().unwrap();
        let inspect = match op1 {
            "*" => match op2 {
                "old" => Square,
                _ => Multer(op2.parse()?)
            },
            "+" => Adder(op2.parse()?),
            _   => unreachable!()
        };

        // Test: divisible by #
        //     If true: throw to monkey #
        //     If false: throw to monkey #
        let_scan!(lines[3]; ("Test: divisible by ", let test_value:usize));
        let_scan!(lines[4]; ("If true: throw to monkey", let target_1:usize));
        let_scan!(lines[5]; ("If false: throw to monkey", let target_2:usize));
        let decide = Decider(test_value, target_1, target_2);

        Ok(Monkey{ items, inspect, decider: decide, activity: 0, })
    }
}

pub fn run_round(monkeys:&mut Vec<Monkey>, ennui:usize, magic:usize) {    
    for idx in 0..monkeys.len() {
        // after all this we should end up with:
        // monkeys_before:&[_], current_monkey, monkeys_upcoming:&[_]
        let (before, rest) = monkeys.split_at_mut(idx);
        if let Some((monkey, after)) = rest.split_first_mut() {
            while monkey.inspect().is_some() {
                monkey.ennui(ennui, magic);
                let catcher = monkey.decide().unwrap();
                let item    = monkey.throw().unwrap();
                match catcher.cmp(&idx) {
                    Less    => { before[catcher].catch(item); },
                    Equal   => { monkey.catch(item); },
                    Greater => { after[catcher - (idx+1)].catch(item); },
                }
            }
        }
    }
}

pub fn get_magic_number(monkeys:&[Monkey]) -> usize {
    monkeys
        .iter()
        .map(|m| if let MonkeyOp::Decider(n,_,_) = m.decider { n } else { 1 })
        .product()
}

pub fn prepare(file_name: &str) -> Result<Vec<Monkey>> {
    let mut monkeys = Vec::new();
    for monkey_section in read_input(file_name).split("\n\n") {
        monkeys.push(monkey_section.parse()?);
    }
    Ok(monkeys)
}

pub fn part_1(mut monkeys:Vec<Monkey>) -> Option<usize> {
    let magic = get_magic_number(&monkeys);
    for _ in 0..20 {
        run_round(&mut monkeys, 3, magic);
    }
    monkeys.sort_by(|a, b| b.activity.cmp(&a.activity));
    Some(monkeys[0].activity * monkeys[1].activity)
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn part_2(mut monkeys: Vec<Monkey>) -> Option<usize> {
    let magic = get_magic_number(&monkeys);
    for _ in 0..10000 {
        run_round(&mut monkeys, 1, magic);
    }
    monkeys.sort_by(|a, b| b.activity.cmp(&a.activity));
    Some(monkeys[0].activity * monkeys[1].activity)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::MonkeyOp::*;

    #[test]
    fn test_round_1() {
        if let Ok(mut monkeys) = prepare("day11-example.txt") {
            let magic = get_magic_number(&monkeys);
            run_round(&mut monkeys, 3, magic);
            assert_eq!(monkeys[0].items,[20, 23, 27, 26]);
            assert_eq!(monkeys[1].items, [2080, 25, 167, 207, 401, 1046]);
            assert!(monkeys[2].items.is_empty());
            assert!(monkeys[3].items.is_empty());
        } else { panic!("No monkeys!")}
    }

    #[test]
    fn test_round_20() {
        if let Ok(mut monkeys) = prepare("day11-example.txt") {
            let magic = get_magic_number(&monkeys);
            for _ in 0..20 {
                run_round(&mut monkeys, 3, magic);
            }
            assert_eq!(monkeys[0].items, [10, 12, 14, 26, 34]);
            assert_eq!(monkeys[1].items, [245, 93, 53, 199, 115]);
            assert!(monkeys[2].items.is_empty());
            assert!(monkeys[3].items.is_empty());
            assert_eq!(monkeys[0].activity * monkeys[3].activity, 10605)
        } else { panic!("No monkeys!")}
    }

    #[test]
    fn test_prepare() {
        if let Ok(monkeys) = prepare("day11-example.txt") {
            assert_eq!(monkeys[0].items, [79, 98]);
            assert_eq!(monkeys[0].inspect, Multer(19));
            assert_eq!(monkeys[0].decider, Decider(23, 2, 3));
            assert_eq!(monkeys[1].inspect, Adder(6));
            assert_eq!(monkeys[2].inspect, Square);
        } else { panic!("No monkeys!")}
    }

    #[test]
    fn test_part_1() {
        if let Ok(monkeys) = prepare("day11-example.txt") {
            assert_eq!(part_1(monkeys), Some(10605))
        } else { panic!("No monkeys!")}
    }

    #[test]
    fn test_part_2() {
        if let Ok(monkeys) = prepare("day11-example.txt") {
            assert_eq!(part_2(monkeys), Some(2713310158))
        } else { panic!("No monkeys!")}
    }
}