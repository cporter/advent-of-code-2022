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

impl ElfPair {
    fn fully_overlaps(&self) -> bool {
        return 
        (self.start_1 <= self.start_2 && self.end_1 >= self.end_2)
        ||
        (self.start_2 <= self.start_1 && self.end_2 >= self.end_1);
    }    

    fn any_overlap(&self) -> bool {
        return
            (self.start_1 <= self.start_2 && self.start_2 <= self.end_1)
            ||
            (self.start_2 <= self.start_1 && self.start_1 <= self.end_2);
    }
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

fn main() {
    let stdin = io::stdin();
    let elves : Vec<ElfPair> = stdin
        .lock()
        .lines()
        .map(|line| from_line(&line.unwrap()))
        .collect();

    let part1 = elves.iter()
        .filter(|e| e.fully_overlaps())
        .count();

    let part2 = elves.iter()
        .filter(|e| e.any_overlap())
        .count();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}