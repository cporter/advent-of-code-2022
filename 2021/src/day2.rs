use std::fmt;
use std::str::FromStr;
use std::io::{self, BufRead};

enum Directive {
    Forward(i32),
    Up(i32),
    Down(i32)
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Directive::Forward(x) => write!(f, "Forward<{}>", x),
	    Directive::Up(x) => write!(f, "Up<{}>", x),
	    Directive::Down(x) => write!(f, "Down<{}>", x),
	}
    }
}

impl FromStr for Directive {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
	let mut split = line.split(" ");
	let directive = split.next();
	let number = split.next();
	let x : i32 = match number {
	    Some(s) => match s.parse() {
		Ok(i) => i,
		Err(why) => panic!("malformed int: {}", why)
	    },
	    None => panic!("malformed (no int): {}", line)
	};
	match directive {
	    Some("forward") => Ok(Directive::Forward(x)),
	    Some("up") => Ok(Directive::Up(x)),
	    Some("down") => Ok(Directive::Down(x)),
	    Some(other_str) => panic!("unknown directive: {}", other_str),
	    None => panic!("malformed (no directive): {:?}", line)
	}
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let stdin = io::stdin();
    let pos = stdin
	.lock()
	.lines()
	.filter_map(|s| s.unwrap().parse::<Directive>().ok())
	.fold(Position{x:0, y:0, aim:0}, |pos, dir| {
	    match dir {
		Directive::Forward(x) => Position{
		    x:pos.x + x,
		    y:pos.y + x * pos.aim,
		    aim:pos.aim
		},
		Directive::Up(x) => Position{
		    x:pos.x,
		    y:pos.y,
		    aim:pos.aim - x
		},
		Directive::Down(x) => Position{
		    x:pos.x,
		    y:pos.y,
		    aim:pos.aim + x
		},
	    }
	});
    println!("{}", pos.x * pos.y);
}
