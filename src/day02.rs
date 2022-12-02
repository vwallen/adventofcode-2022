use adventofcode_2022::read_input;

pub fn prepare(file_name:&str) -> Vec<(&str, &str)> {
    let input = read_input(file_name);
    let mut output = Vec::new();
    for line in input.lines() {
        output.push(match line {
            "A X" => ("A", "X"),
            "B X" => ("B", "X"),
            "C X" => ("C", "X"),
            "A Y" => ("A", "Y"),
            "B Y" => ("B", "Y"),
            "C Y" => ("C", "Y"),
            "A Z" => ("A", "Z"),
            "B Z" => ("B", "Z"),
            "C Z" => ("C", "Z"),
            _ => ("", "")
        })
    }
    output
}

pub fn part_1(input: &Vec<(&str, &str)>) -> Option<u32> {
    let mut score = 0;
    for game in input.iter() {
        match game {
            ("A", "X") => score += 1 + 3,
            ("B", "X") => score += 1 + 0,
            ("C", "X") => score += 1 + 6,
            ("A", "Y") => score += 2 + 6,
            ("B", "Y") => score += 2 + 3,
            ("C", "Y") => score += 2 + 0,
            ("A", "Z") => score += 3 + 0,
            ("B", "Z") => score += 3 + 6,
            ("C", "Z") => score += 3 + 3,
            _ => score += 0
        }
    }
    Some(score)
}

pub fn part_2(input: &Vec<(&str, &str)>) -> Option<u32> {
    let mut score = 0;
    for game in input.iter() {
        match game {
            ("A", "X") => score += 3 + 0,
            ("B", "X") => score += 1 + 0,
            ("C", "X") => score += 2 + 0,
            ("A", "Y") => score += 1 + 3,
            ("B", "Y") => score += 2 + 3,
            ("C", "Y") => score += 3 + 3,
            ("A", "Z") => score += 2 + 6,
            ("B", "Z") => score += 3 + 6,
            ("C", "Z") => score += 1 + 6,
            _ => score += 0
        }
    }
    Some(score)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day02-example.txt");
        assert_eq!(input[0], ("A", "Y"))
    }

    #[test]
    fn test_part_1() {
        let input = prepare("day02-example.txt");
        assert_eq!(part_1(&input), Some(15))
    }

    #[test]
    fn test_part_2() {
        let input = prepare("day02-example.txt");
        assert_eq!(part_2(&input), Some(12))
    }

}