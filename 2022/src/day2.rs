use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = i32;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line.trim() {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scissors),
            _ => Err(-1),
        }
    }
}

#[derive(Debug)]
struct Game {
    them: Hand,
    us: Hand,
}

impl FromStr for Game {
    type Err = i32;
    fn from_str(line: &str) -> Result<Game, Self::Err> {
        let mut split = line.trim().split(" ");
        let them = split.next();
        let us = split.next();
        return Ok(Game {
            them: them.unwrap().parse().unwrap(),
            us: us.unwrap().parse().unwrap(),
        });
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Tie,
}

fn score_game(g: &Game) -> GameResult {
    match g.them {
        Hand::Rock => {
            match g.us {
                Hand::Rock => GameResult::Tie,
                Hand::Paper => GameResult::Win,
                Hand::Scissors => GameResult::Lose,
            }
        }
        Hand::Paper => {
            match g.us {
                Hand::Rock => GameResult::Lose,
                Hand::Paper => GameResult::Tie,
                Hand::Scissors => GameResult::Win,
            }
        }
        Hand::Scissors => {
            match g.us {
                Hand::Rock => GameResult::Win,
                Hand::Paper => GameResult::Lose,
                Hand::Scissors => GameResult::Tie,
            }
        }
    }
}

fn score_game_result(r: GameResult) -> i32 {
    match r {
        GameResult::Win => 6,
        GameResult::Tie => 3,
        GameResult::Lose => 0,
    }
}

fn score_hand(h: &Hand) -> i32 {
    match h {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn score(g: &Game) -> i32 {
    return score_hand(&g.us) + score_game_result(score_game(&g));
}

fn day2_outcome(h: Hand) -> GameResult {
    match h {
        Hand::Rock => GameResult::Lose,
        Hand::Paper => GameResult::Tie,
        Hand::Scissors => GameResult::Win,
    }
}

fn day2_move(g: &Game, r: &GameResult) -> Hand {
    match r {
        GameResult::Tie => g.them,
        GameResult::Win => {
            match g.them {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            }
        }
        GameResult::Lose => {
            match g.them {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            }
        }
    }
}

fn day2_score(g: &Game) -> i32 {
    let outcome = day2_outcome(g.us);
    let mv = day2_move(g, &outcome);
    return score_hand(&mv) + score_game_result(outcome);
}

fn main() {
    let stdin = io::stdin();
    let games: Vec<Game> = stdin
        .lock()
        .lines()
        .filter_map(|line| {
            let s = line.unwrap();
            return s.parse::<Game>().ok();
        })
        .collect();

    let part1_score: i32 = games.iter().map(|g| score(g)).sum();

    let part2_score: i32 = games.iter().map(|g| day2_score(&g)).sum();

    println!("part 1: {}", part1_score);
    println!("part 2: {}", part2_score);
}
