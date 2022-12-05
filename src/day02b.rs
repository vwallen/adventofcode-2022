use adventofcode_2022::read_input_lines;
use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

const PRECEDENCE:[Play; 5] = [Play::Rock, Play::Paper, Play::Scissors, Play::Rock, Play::Paper];

pub fn prepare(file_name:&str) -> Vec<(String, String)> {
    read_input_lines(file_name)
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|c| c.to_string())
                .next_tuple().unwrap()
        })
        .collect()
}

pub fn decode_play(play_code: &str) -> Play {
    match play_code {
        "A"|"X" => Play::Rock,
        "B"|"Y" => Play::Paper,
        "C"|"Z" => Play::Scissors,
        _ => unreachable!()
    }    
}

pub fn decode_outcome(outcome_code: &str) -> Outcome {
    match outcome_code {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!()
    }    
}

pub fn calculate_score_1(p1:Play, p2:Play) -> u16 {
    if p1 == p2 {
        p1 as u16 + Outcome::Draw as u16
    } else {
        match (p1, p2) {
            (Play::Rock, Play::Scissors) => Play::Rock as u16 + Outcome::Win as u16,
            (Play::Paper, Play::Rock) => Play::Paper as u16 + Outcome::Win as u16,
            (Play::Scissors, Play::Paper) => Play::Scissors as u16 + Outcome::Win as u16,                 
            (p1, _) => p1 as u16 + Outcome::Lose as u16
        }
    }
}

pub fn calculate_score_2(p1:Play, outcome:Outcome) -> u16 {
    let play = p1 as usize;
    outcome as u16 + match outcome {
        Outcome::Win => PRECEDENCE[play] as u16,
        Outcome::Lose => PRECEDENCE[play + 1] as u16,
        Outcome::Draw => p1 as u16,
    }
}

pub fn part_1(input: &[(String, String)]) -> Option<u16> {
    let mut score = 0;
    for result in input.iter() {
        score += calculate_score_1(
            decode_play(result.1.as_str()),
            decode_play(result.0.as_str()),
        );
    }
    Some(score)
}

pub fn part_2(input: &[(String, String)]) -> Option<u16> {
    let mut score = 0;
    for result in input.iter() {
        score += calculate_score_2(
            decode_play(result.0.as_str()),
            decode_outcome(result.1.as_str()),
        );
    }
    Some(score)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        let input = prepare("day02-example.txt");
        assert_eq!(input[0], ("A".to_string(), "Y".to_string()))
    }

    #[test]
    fn test_calculate_score_1() {
        assert_eq!(calculate_score_1(Play::Rock, Play::Scissors), Play::Rock as u16 + Outcome::Win as u16);        
        assert_eq!(calculate_score_1(Play::Paper, Play::Rock), Play::Paper as u16 + Outcome::Win as u16);        
        assert_eq!(calculate_score_1(Play::Scissors, Play::Rock), Play::Scissors as u16 + Outcome::Lose as u16);      
        assert_eq!(calculate_score_1(Play::Rock, Play::Rock), Play::Rock as u16 + Outcome::Draw as u16)        
    }

    #[test]
    fn test_calculate_score_2() {
        assert_eq!(calculate_score_2(Play::Scissors, Outcome::Win), Play::Rock as u16 + Outcome::Win as u16);
        assert_eq!(calculate_score_2(Play::Rock, Outcome::Lose), Play::Scissors as u16 + Outcome::Lose as u16)
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