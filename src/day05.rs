use adventofcode_2022::read_input;
use itertools::Itertools;
use anyhow::{Result};
use std::str::FromStr;

pub struct SupplyCrate(char);

pub enum MoveHow {
    Singly,
    AtOnce,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MoveCommand {
    src: usize,
    dst: usize,
    amt: usize,
}
impl FromStr for MoveCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // this seems overly convoluted and terrible
        let (amt, src, dst) = 
            s.split(' ')
            .map(|w| w.parse::<usize>())
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .next_tuple()
            .unwrap();
        Ok(Self{ src: src - 1, dst: dst - 1, amt })
    }
}


#[derive(Default)]
pub struct Inventory {
    stacks: Vec<Vec<SupplyCrate>>,
}
impl Inventory {

    fn add(&mut self, pos: usize) {
        if self.stacks.get_mut(pos).is_none() {
            self.stacks.insert(pos, Vec::new());
        }
    }

    fn place(&mut self, item: SupplyCrate, pos: usize) {
        self.stacks[pos].push(item);
    }

    fn move_crate(&mut self, src: usize, dst: usize, amount: usize, how:MoveHow) {
        use MoveHow::*;
        match how {
            Singly => for _ in 0..amount {
                let item = self.stacks[src].pop().unwrap();
                self.stacks[dst].push(item);
            },
            AtOnce => {
                let stack_from = &mut self.stacks[src];
                let items = stack_from.split_off(stack_from.len() - amount);
                for item in items.into_iter() {
                    self.stacks[dst].push(item);
                }
            },
        }
    }

    fn peek(&self) -> String {
        self.stacks.iter().map(|x| x.last().unwrap().0).join("")
    }
}

//--------------------------------------------------

pub fn prepare(file_name:&str) -> Result<(Inventory, Vec<MoveCommand>)> {
    let input = read_input(file_name);
    let (_inventory_section, command_section) = input.split_once("\n\n").unwrap();
    
    let mut inventory = Inventory::default();
    inventory.add(0);
    inventory.add(1);
    inventory.add(2);
    inventory.place(SupplyCrate('Z'), 0);
    inventory.place(SupplyCrate('N'), 0);
    inventory.place(SupplyCrate('M'), 1);
    inventory.place(SupplyCrate('C'), 1);
    inventory.place(SupplyCrate('D'), 1);
    inventory.place(SupplyCrate('P'), 2);

    let commands = command_section.lines().map(|line| line.parse().unwrap()).collect();

    Ok((inventory, commands))
}


pub fn part_1(inventory:&mut Inventory, commands: Vec<MoveCommand>) -> Option<String> {
    use MoveHow::*;

    for cmd in commands.iter() {
        inventory.move_crate(cmd.src, cmd.dst, cmd.amt, Singly)
    }

    Some(inventory.peek())
}

pub fn part_2(inventory:&mut Inventory, commands: Vec<MoveCommand>) -> Option<String> {
    use MoveHow::*;

    for cmd in commands.iter() {
        inventory.move_crate(cmd.src, cmd.dst, cmd.amt, AtOnce)
    }

    Some(inventory.peek())
}

//--------------------------------------------------

#[cfg(test)]
mod test {

    use super::*;
    use super::MoveHow::*;


    #[test]
    #[should_panic]
    fn test_invalid_placement() {
        let mut inventory = Inventory::default();
        inventory.place(SupplyCrate('A'), 0);
    }

    #[test]
    fn test_valid_move() {
        let mut inventory = Inventory::default();
        inventory.add(0);
        inventory.add(1);
        inventory.place(SupplyCrate('A'), 0);
        inventory.move_crate(0, 1, 1, Singly);
    }

    #[test]
    #[should_panic]
    fn test_invalid_move() {
        let mut inventory = Inventory::default();
        inventory.add(0);
        inventory.place(SupplyCrate('A'), 0);
        inventory.move_crate(0, 1, 1, Singly);
    }

    #[test]
    fn test_prepare() {
        let (_inventory, commands) = prepare("day05-example.txt").unwrap();
        assert_eq!(commands[0], MoveCommand{ src: 1, dst: 0, amt: 1});
        assert_eq!(commands[3], MoveCommand{ src: 0, dst: 1, amt: 1});
    }

    #[test]
    fn test_part_1() {
        let (mut inventory, commands) = prepare("day05-example.txt").unwrap();
        assert_eq!(part_1(&mut inventory, commands), Some("CMZ".to_string()))
    }

    #[test]
    fn test_part_2() {
        let (mut inventory, commands) = prepare("day05-example.txt").unwrap();
        assert_eq!(part_2(&mut inventory, commands), Some("MCD".to_string()))
    }

}