#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::io::{self, BufRead};

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move(s: &str) -> Move {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let mut caps = RE.captures_iter(s);
    let cap = caps.next().unwrap();
    return Move {
        count: cap[1].parse().unwrap(),
        from: cap[2].parse::<usize>().unwrap() - 1,
        to: cap[3].parse::<usize>().unwrap() - 1,
    };
}

#[derive(Eq, PartialEq)]
enum ParseState {
    Board,
    Moves,
}

fn apply_move(mv: &Move, s: &mut Stacks) {
    (0..mv.count).for_each(|_| {
        let x = s[mv.from].pop().unwrap();
        s[mv.to].push(x);
    });
}

fn apply_move2(mv: &Move, s: &mut Stacks) {
    apply_move(mv, s);
    let n = s[mv.to].len();
    s[mv.to][(n - mv.count)..n].reverse();
}

fn main() {
    let input = io::stdin().lock().lines();
    let lines = input.into_iter();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks2: Vec<Vec<char>> = Vec::new();
    let mut state = ParseState::Board;

    for rline in lines {
        let line = rline.unwrap();
        if 0 == line.trim().len() {
            state = ParseState::Moves;
            for s in stacks.iter_mut() {
                s.reverse();
            }
            stacks2 = stacks.iter().map(|v| v.to_vec()).collect();
        } else if ParseState::Board == state {
            line.chars().enumerate().for_each(|(i, ch)| if 'A' <= ch &&
                ch <= 'Z'
            {
                let index = i / 4;
                while stacks.len() <= index {
                    stacks.push(Vec::new());
                }
                stacks[index].push(ch);
            })
        } else if ParseState::Moves == state {
            let mv = parse_move(&line);
            apply_move(&mv, &mut stacks);
            apply_move2(&mv, &mut stacks2)
        }
    }

    let part1: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    let part2: String = stacks2.iter().map(|s| s.last().unwrap()).collect();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
