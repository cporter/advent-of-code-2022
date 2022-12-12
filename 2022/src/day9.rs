use std::io::BufRead;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug)]
enum Move {
    Left(u8),
    Right(u8),
    Up(u8),
    Down(u8),
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
        match cap[1].chars().next() {
            Some('L') => Ok(Move::Left(n)),
            Some('R') => Ok(Move::Right(n)),
            Some('U') => Ok(Move::Up(n)),
            Some('D') => Ok(Move::Down(n)),
            _ => panic!("Unexpected input: {}", line)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x : i32,
    y : i32,
}

fn dist(a : &Point, b : &Point) -> i32 {
    max((a.x - b.x).abs(), (a.y - b.y).abs())
}

fn move_head(a : &Point, m : &Move) -> Point {
    match m {
        Move::Left(_) => Point { x: a.x - 1, y: a.y },
        Move::Right(_) => Point { x: a.x + 1, y: a.y },
        Move::Up(_) => Point { x: a.x, y: a.y + 1 },
        Move::Down(_) => Point { x: a.x, y: a.y - 1 },
    }
}

// move a towards b if it's not touching
fn move_tail(a : &Point, b : &Point, m : &Move) -> Point {
    if 1 < dist(a, b) {
        match m {
            Move::Left(_) => Point { x: b.x + 1, y: b.y },
            Move::Right(_) => Point { x: b.x - 1, y: b.y },
            Move::Up(_) => Point { x: b.x, y: b.y - 1 },
            Move::Down(_) => Point { x: b.x, y: b.y + 1 },
        }
    } else {
        *a
    }
}

fn main() {
    let mut h = Point { x: 0, y: 0 };
    let mut t = Point { x: 0, y: 0 };
    let mut moves : HashSet<Point> = HashSet::new();
    moves.insert(t);

    std::io::stdin()
        .lock()
        .lines()
        .filter_map(|s| s.unwrap().parse::<Move>().ok())
        .for_each(|m| {
            let c = match m {
                Move::Left(x) => x,
                Move::Right(x) => x,
                Move::Up(x) => x,
                Move::Down(x) => x,
            };
            for _ in 0..c {
                h = move_head(&h, &m);
                t = move_tail(&t, &h, &m);
                moves.insert(t);
            }
        });

    println!("part 1: {}", moves.len());
}