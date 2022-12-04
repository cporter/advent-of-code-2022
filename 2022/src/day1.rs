use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut elves: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    stdin.lock().lines().for_each(|line| {
        let s = line.unwrap();
        if 0 == s.trim().len() {
            elves.push(total);
            total = 0;
        } else {
            match s.parse::<i32>() {
                Ok(i) => total += i,
                Err(why) => panic!("invalid input: {}", why),
            }
        }
    });
    elves.push(total);
    elves.sort();

    let n: usize = elves.len();

    println!("part 1: {}", elves[n - 1]);
    println!("part 2: {}", elves[n - 1] + elves[n - 2] + elves[n - 3]);
}
