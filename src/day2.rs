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


fn day2_part1() -> u32 {
    let input = common::get_file("src/input/day2.txt").expect("failed to read file");
    let rounds: Vec<(&str, &str)> = input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();
    let mut score: u32 = 0;

    for round in rounds {
        match round {
            (OPPONENT_ROCK, PAPER) => add_win_score(&mut score),
            (OPPONENT_PAPER, SCISSORS) => add_win_score(&mut score),
            (OPPONENT_SCISSORS, ROCK) => add_win_score(&mut score),
            (OPPONENT_PAPER, PAPER) => add_draw_score(&mut score),
            (OPPONENT_SCISSORS, SCISSORS) => add_draw_score(&mut score),
            (OPPONENT_ROCK, ROCK) => add_draw_score(&mut score),
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

fn day2_part2() -> u32 {
    let mut score = 0;
    let input = common::get_file("src/input/day2.txt").expect("failed to read file");
    let rounds: Vec<(&str, &str)> = input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();
    for round in rounds {
        match round.1 {
            "Z" => add_win_score(&mut score),
            "Y" => add_draw_score(&mut score),
            _ => ()
        }

        match round.1 {
            "X" => {
                match round.0 {
                    OPPONENT_ROCK => score += SCISSORS_SCORE,
                    OPPONENT_SCISSORS => score += PAPER_SCORE,
                    OPPONENT_PAPER => score += ROCK_SCORE,
                    _ => ()
                }
            }
            "Y" => {
                match round.0 {
                    OPPONENT_ROCK => score += ROCK_SCORE,
                    OPPONENT_SCISSORS => score += SCISSORS_SCORE,
                    OPPONENT_PAPER => score += PAPER_SCORE,
                    _ => ()
                }
            }
            "Z" => {
                match round.0 {
                    OPPONENT_ROCK => score += PAPER_SCORE,
                    OPPONENT_SCISSORS => score += ROCK_SCORE,
                    OPPONENT_PAPER => score += SCISSORS_SCORE,
                    _ => ()
                }
            }
            _ => ()
        }
    }
    score
}

fn add_draw_score(score: &mut u32) {
    *score += DRAW
}

fn add_win_score(score: &mut u32) {
    *score += WIN
}

#[cfg(test)]
mod tests {
    use crate::day2::{day2_part1, day2_part2};

    #[test]
    fn day2_part1_test() {
        assert_eq!(day2_part1(), 14375)
    }

    #[test]
    fn day2_part2_test() {
        assert_eq!(day2_part2(), 10274)
    }
}

