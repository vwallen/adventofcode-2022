
use adventofcode_2022::read_input;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<String> {
    let input = read_input(file_name);
    Ok(input)
}

pub fn part_1(_input: &str) -> Option<usize> {
    None
}

pub fn part_2(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn test_part_1() {
        if let Ok(input) = prepare("day08-example.txt") {
            assert_eq!(part_1(&input), Some(1))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day08-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}