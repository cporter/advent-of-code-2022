use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut elves : Vec<i32> = Vec::new();
    let mut total : i32 = 0;

    stdin
        .lock()
        .lines()
        .for_each(|s| {
            match s.unwrap().parse::<i32>() {
                Ok(i) => total += i,
                Err(_why) => {
                    elves.push(total);
                    total = 0;
                }
            }
        });
    elves.push(total);
    elves.sort();
    
    let n : usize = elves.len();

    println!("part 1: {}", elves[n-1]);
    println!("part 2: {}", elves[n-1] + elves[n-2] + elves[n-3]);
}  
