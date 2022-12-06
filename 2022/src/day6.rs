use std::io::{self};
use std::collections::HashSet;

fn main() {
    let stdin = io::read_to_string(io::stdin()).unwrap();

    for i in 4..stdin.len() {
        let current = stdin[(i-4)..(i)].chars().collect::<HashSet<_>>();
        if 4 == current.len() {
            println!("part 1: {}", i);
            break;
        }
    }
}