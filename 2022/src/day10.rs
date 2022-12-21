use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(line_raw: &str) -> Result<Instruction, Self::Err> {
        let line = line_raw.trim();
        if "noop" == line {
            Ok(Instruction::Noop)
        } else {
            let s : Vec<&str> = line.split(" ").collect();
            let n : i32 = s[1].parse().unwrap();

            Ok(Instruction::Addx(n))
        }
    }
}

struct Add {
    value: i32,
    cycles: i32,
}

fn main() {
    let mut part1 : i32 = 0;
    let mut x : i32 = 1;
    let mut cycle = 1;
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(|s| s.unwrap().parse::<Instruction>().ok())
        .map(|payload| {
            match payload {
                Instruction::Noop => Add { value: 0, cycles: 1 },
                Instruction::Addx(n) => Add { value: n, cycles: 2 },
            }
        })
        .for_each(|add| {
            for _ in 0..add.cycles {
                if 0 == (cycle - 20) % 40 {
                    println!("********* {cycle} {x} {}", cycle * x);
                    part1 += cycle * x;    
                }
                cycle += 1;
            }
            x += add.value;
        });

    println!("part 1: {part1}"); // 15680 is too high
}