#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct ElfPair {
    start_1: i32,
    end_1: i32,
    start_2: i32,
    end_2: i32,
}

impl ElfPair {
    fn fully_overlaps(&self) -> bool {
        return (self.start_1 <= self.start_2 && self.end_1 >= self.end_2)
            || (self.start_2 <= self.start_1 && self.end_2 >= self.end_1);
    }

    fn any_overlap(&self) -> bool {
        return (self.start_1 <= self.start_2 && self.start_2 <= self.end_1)
            || (self.start_2 <= self.start_1 && self.start_1 <= self.end_2);
    }
}

impl FromStr for ElfPair {
    type Err = String;
    fn from_str(line: &str) -> Result<ElfPair, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }
        let mut caps = RE.captures_iter(line);
        let cap = caps.next().unwrap();
        return Ok(ElfPair {
            start_1: cap[1].parse().unwrap(),
            end_1: cap[2].parse().unwrap(),
            start_2: cap[3].parse().unwrap(),
            end_2: cap[4].parse().unwrap(),
        });
    }
}

fn main() {
    let stdin = io::stdin();
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    stdin
        .lock()
        .lines()
        .filter_map(|line| line.unwrap().parse::<ElfPair>().ok())
        .for_each(|e| {
            if e.fully_overlaps() {
                part1 += 1;
            }
            if e.any_overlap() {
                part2 += 1;
            }
        });

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
