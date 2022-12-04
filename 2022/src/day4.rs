#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::io::{self, BufRead};

#[derive(Debug)]
struct ElfPair {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
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
        a: a,
        b: b,
        c: c,
        d: d,
    }
}

fn fully_overlaps(e: &ElfPair) -> bool {
    return 
        (e.a <= e.c && e.a <= e.d && e.b >= e.c && e.b >= e.d)
        ||
        (e.c <= e.a && e.c <= e.b && e.d >= e.a && e.d >= e.b);
}

fn any_overlap(e: &ElfPair) -> bool {
    return
        (e.a <= e.c && e.c <= e.b)
        ||
        (e.c <= e.a && e.a <= e.d);
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