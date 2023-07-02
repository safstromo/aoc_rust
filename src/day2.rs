use std::ops::Add;
use crate::common;


const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const ROCK: &str = "X";
const PAPER: &str = "Y";
const SCISSORS: &str = "Z";


const WIN: u32 = 6;
const DRAW: u32 = 3;
const SCISSORS_SCORE: u32 = 3;
const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;


fn day2() -> u32 {
    let input = common::get_file("src/input/day2.txt").expect("failed to read file");
    let rounds: Vec<(&str, &str)> = input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();
    let mut score: u32 = 0;

    for round in rounds {
        match round {
            (OPPONENT_ROCK, PAPER) => score += WIN,
            (OPPONENT_PAPER, SCISSORS) => score += WIN,
            (OPPONENT_SCISSORS, ROCK) => score += WIN,
            (OPPONENT_PAPER, PAPER) => score += DRAW,
            (OPPONENT_SCISSORS, SCISSORS) => score += DRAW,
            (OPPONENT_ROCK, ROCK) => score += DRAW,
            _ => ()
        }

        match round.1 {
            PAPER => score += PAPER_SCORE,
            ROCK => score += ROCK_SCORE,
            SCISSORS => score += SCISSORS_SCORE,
            _ => ()
        }
    }

    score
}


mod tests {
    use crate::day2::day2;

    #[test]
    fn day2_part1_test() {
        assert_eq!(day2(), 14375)
    }
}
