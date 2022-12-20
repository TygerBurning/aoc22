use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn contest(opponent: Choice, me: Choice) -> GameResult {
    if opponent as usize == me as usize {
        GameResult::Draw
    } else if opponent as usize == me as usize % 3 + 1 {
        GameResult::Lose
    } else {
        GameResult::Win
    }
}

fn i_should_pick(opponent: Choice, result: GameResult) -> Choice {
    // Bleugh
    match result {
        GameResult::Win => Choice::from_u32(opponent as u32 % 3 + 1),
        GameResult::Draw => opponent,
        GameResult::Lose => Choice::from_u32((opponent as u32 + 1) % 3 + 1),
    }
}

impl Choice {
    fn from_u32(value: u32) -> Choice {
        match value {
            1 => Choice::Rock,
            2 => Choice::Paper,
            3 => Choice::Scissors,
            _ => panic!("Unknown value: {}", value),
        }
    }

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for GameResult {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParseError),
        }
    }
}

fn score_game_strategy_1(game: &str) -> u32 {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    let cap = re.captures_iter(game).next().unwrap();
    let opponent = Choice::from_str(&cap[1]).unwrap();
    let me = Choice::from_str(&cap[2]).unwrap();
    me as u32 + contest(opponent, me) as u32
}

fn score_game_strategy_2(game: &str) -> u32 {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    let cap = re.captures_iter(game).next().unwrap();
    let opponent = Choice::from_str(&cap[1]).unwrap();
    let result = GameResult::from_str(&cap[2]).unwrap();
    result as u32 + i_should_pick(opponent, result) as u32
}

pub fn day02() {
    let games = include_str!("../inputs/day02.txt").lines();

    let scores1 = games.clone().map(score_game_strategy_1);
    let scores2 = games.map(score_game_strategy_2);

    println!("Part A answer is: {:?}", scores1.sum::<u32>());
    println!("Part B answer is: {:?}", scores2.sum::<u32>());
}
