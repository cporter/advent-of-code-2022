use std::io::BufRead;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::cmp::min;
use std::collections::HashSet;
use std::ops;



#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x : i32,
    y : i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs : Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs : Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct Move {
    v: Point,
    count: u8,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(line: &str) -> Result<Move, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([LRUD])\s+(\d+)").unwrap();
        }
        let mut caps = RE.captures_iter(line);
        let cap = caps.next().unwrap();
        let n : u8 = cap[2].parse().unwrap();
        Ok(Move {
            v: match cap[1].chars().next() {
                Some('L') => Point {x: -1, y: 0},
                Some('R') => Point {x: 1, y: 0},
                Some('U') => Point {x: 0, y: 1},
                Some('D') => Point {x: 0, y: -1},
                _ => panic!("Unexpected input: {}", line)
            },
            count: n,
        })
    }
}

fn sign(x: i32) -> i32 {
    if x < 0 {
        -1
    } else {
        1
    }
}

// had a little trouble groking the problem with this one and ended
// up with a solution off of the internets. Oh well!

fn update_tail(head : Point, tail : Point, prev_head: Point) -> Point {
    let delta = head - tail;
    if 2 > delta.x.abs() && 2 > delta.y.abs() {
        tail
    } else {
        if prev_head.x == tail.x || prev_head.y == tail.y {
            tail + (head - prev_head)
        } else {
            let diff = head - tail;

            Point {
                x: tail.x + sign(diff.x) * min(1,diff.x.abs()),
                y: tail.y + sign(diff.y) * min(1,diff.y.abs()),
            }
        }
    }
}

fn solve(moves: &Vec<Move>, rope_size : usize) -> usize {
    let mut current = vec![Point{x:0, y:0}; rope_size];
    let mut last_link_history: HashSet<Point> = HashSet::new();
    moves.iter().for_each(|mv| {
        for _ in 0..mv.count {
            let mut prev_tail = current[0];
            current[0] = current[0] + mv.v;
            for tail_num in 0..(current.len()-1) {
                let next_prev_tail = current[1 + tail_num];
                current[1 + tail_num] = update_tail(
                    current[tail_num], 
                    current[tail_num + 1],
                    prev_tail);
                prev_tail = next_prev_tail;
            }
            last_link_history.insert(current[current.len()-1]);
        }
    });
    last_link_history.len()
}

fn main() {
    let moves : Vec<Move> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|s| s.unwrap().parse::<Move>().ok())
        .collect();

    println!("part 1: {}", solve(&moves, 2));
    println!("part 2: {}", solve(&moves, 10)); // 2588: too low
}

