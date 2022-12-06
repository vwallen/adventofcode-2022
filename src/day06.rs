
use adventofcode_2022::read_input;
use anyhow::Result;
use itertools::Itertools;

pub fn prepare(file_name: &str) -> Result<String> {
    let input = read_input(file_name);
    Ok(input)
}

pub fn find_marker(input: &str, range: usize) -> Option<usize> {
    let stream:Vec<char> = input.chars().collect();
    for (i, window) in stream.windows(range).enumerate() {
        if window.iter().all_unique() {
            return Some(i + range)
        }
    }
    None    
}

pub fn part_1(input: &str) -> Option<usize> {
    find_marker(input, 4)
}

pub fn part_2(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), Some(7));
        assert_eq!(part_1(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), Some(5));
        assert_eq!(part_1(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), Some(6));
        assert_eq!(part_1(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), Some(10));
        assert_eq!(part_1(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), Some(11));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), Some(19));
        assert_eq!(part_2(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), Some(23));
        assert_eq!(part_2(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), Some(23));
        assert_eq!(part_2(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), Some(29));
        assert_eq!(part_2(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), Some(26));
    }
}