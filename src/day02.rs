use adventofcode_2022::read_input_lines;

const ROCK:u32 = 1;
const PAPER:u32 = 2;
const SCISSORS:u32 = 3;
const WIN:u32 = 6;
const DRAW:u32 = 3;
const LOSE:u32 = 0;

pub fn prepare(file_name:&str) -> Vec<String> {
    read_input_lines(file_name)
}

pub fn part_1(input: &Vec<String>) -> Option<u32> {
    let mut score = 0;
    for game in input.iter() {
        match game.as_str() {
            "A X" => score += ROCK + DRAW,
            "B X" => score += ROCK + LOSE,
            "C X" => score += ROCK + WIN,
            "A Y" => score += PAPER + WIN,
            "B Y" => score += PAPER + DRAW,
            "C Y" => score += PAPER + LOSE,
            "A Z" => score += SCISSORS + LOSE,
            "B Z" => score += SCISSORS + WIN,
            "C Z" => score += SCISSORS + DRAW,
            _ => score += 0
        }
    }
    Some(score)
}

pub fn part_2(input: &Vec<String>) -> Option<u32> {
    let mut score = 0;
    for game in input.iter() {
        match game.as_str() {
            "A X" => score += SCISSORS + LOSE,
            "B X" => score += ROCK + LOSE,
            "C X" => score += PAPER + LOSE,
            "A Y" => score += ROCK + DRAW,
            "B Y" => score += PAPER + DRAW,
            "C Y" => score += SCISSORS + DRAW,
            "A Z" => score += PAPER + WIN,
            "B Z" => score += SCISSORS + WIN,
            "C Z" => score += ROCK + WIN,
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
        assert_eq!(input[0], "A Y".to_string())
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