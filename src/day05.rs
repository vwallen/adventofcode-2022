use adventofcode_2022::read_input;
use itertools::Itertools;
use anyhow::{Result};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct SupplyCrate(char);

pub enum MoveHow {
    Singly,
    AtOnce,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MoveCommand {
    src: usize,
    dst: usize,
    amt: u8,
}
impl FromStr for MoveCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let_scan!(s; ("move ", let amt:u8, " from ", let src:usize, " to ", let dst:usize));
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

    fn move_crate(&mut self, src: usize, dst: usize, amount: u8, how:MoveHow) {
        use MoveHow::*;
        match how {
            Singly => for _ in 0..amount {
                let item = self.stacks[src].pop().unwrap();
                self.stacks[dst].push(item);
            },
            AtOnce => {
                let stack_from = &mut self.stacks[src];
                let stack_index:i16 = (stack_from.len() - amount as usize).try_into().unwrap();
                let items = stack_from.split_off(stack_index.try_into().unwrap());
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
impl FromStr for Inventory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut me = Inventory::default();

        let mut inv = s.lines().rev();
        if let Some(header_line) = inv.next() {
            for i in 0..=(header_line.len()/4) {
                me.add(i as usize);
            }
        }

        for stack_line in inv {
            for (i, ch) in stack_line.chars().enumerate() {
                if let Some(krate) = match ch {
                    ' '|'['|']' => None,
                              _ => Some(ch)
                } {
                    me.place(SupplyCrate(krate), i/4);
                }
            }
        }
        Ok(me)
    }
}

//--------------------------------------------------

pub fn prepare(file_name:&str) -> Result<(Inventory, Vec<MoveCommand>)> {
    let input = read_input(file_name);
    let (inventory_section, command_section) = input.split_once("\n\n").unwrap();

    let inventory:Inventory = inventory_section.parse().unwrap();
    let commands = command_section.lines().map(|line| line.parse().unwrap()).collect();

    Ok((inventory, commands))
}

pub fn part_1(inventory:&mut Inventory, commands: Vec<MoveCommand>) -> Option<String> {
    for cmd in commands.iter() {
        inventory.move_crate(cmd.src, cmd.dst, cmd.amt, MoveHow::Singly)
    }
    Some(inventory.peek())
}

pub fn part_2(inventory:&mut Inventory, commands: Vec<MoveCommand>) -> Option<String> {
    for cmd in commands.iter() {
        inventory.move_crate(cmd.src, cmd.dst, cmd.amt, MoveHow::AtOnce)
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
        let (inventory, commands) = prepare("day05-example.txt").unwrap();
        assert_eq!(inventory.stacks[0][0], SupplyCrate('Z'));
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