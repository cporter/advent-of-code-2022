use std::collections::HashSet;
use std::io::{self};

fn main() {
    let stdin = io::read_to_string(io::stdin()).unwrap();

    let mut part1: Option<usize> = None;
    let mut part2: Option<usize> = None;

    for i in 4..stdin.len() {
        match part1 {
            Some(_) => {},
            None => {
                let packet = stdin[(i - 4)..(i)].chars().collect::<HashSet<_>>();
                if 4 == packet.len() {
                    part1 = Some(i)
                }
            }
        }
        if 14 <= i {
            let message = stdin[(i - 14)..i].chars().collect::<HashSet<_>>();
            if 14 == message.len() {
                part2 = Some(i);
                break;
            }
        }
    }

    println!("part 1: {}", part1.unwrap());
    println!("part 2: {}", part2.unwrap());
}
