#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::io::{self, BufRead};
use std::cmp::{min, max};

#[derive(Debug)]
struct ElfPair {
    start_1 : i32,
    end_1 : i32,
    start_2 : i32,
    end_2 : i32,
}

fn from_line(s: &str) -> ElfPair {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let mut caps = RE.captures_iter(s);
    let cap = caps.next().unwrap();
    let a : i32 = cap[1].parse().unwrap();
    let b : i32 = cap[2].parse().unwrap();
    let c : i32 = cap[3].parse().unwrap();
    let d : i32 = cap[4].parse().unwrap();
    return ElfPair {
        start_1: min(a, b),
        end_1: max(a, b),
        start_2: min(c, d),
        end_2: max(c, d),
    }
}

fn fully_overlaps(e: &ElfPair) -> bool {
    return 
        (e.start_1 <= e.start_2 && e.end_1 >= e.end_2)
        ||
        (e.start_2 <= e.start_1 && e.end_2 >= e.end_1);
}

fn any_overlap(e: &ElfPair) -> bool {
    return
        (e.start_1 <= e.start_2 && e.start_2 <= e.end_1)
        ||
        (e.start_2 <= e.start_1 && e.start_1 <= e.end_2);
}

fn main() {
    let stdin = io::stdin();
    let elves : Vec<ElfPair> = stdin
        .lock()
        .lines()
        .map(|line| from_line(&line.unwrap()))
        .collect();

    let part1 = elves.iter()
        .filter(|e| fully_overlaps(e))
        .count();

    let part2 = elves.iter()
        .filter(|e| any_overlap(e))
        .count();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}