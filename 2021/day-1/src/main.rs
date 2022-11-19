use std::io::{self, BufRead};
use std::iter::zip;

fn main() {
    let stdin = io::stdin();
    let v : Vec<i32> = stdin
	.lock()
	.lines()
	.filter_map(|s| s.unwrap().parse().ok())
	.collect();

    let head = v.iter();
    let tail = v.iter().skip(1);

    let increase = zip(head, tail)
	.filter(|(prev, succ)| prev < succ)
	.count();

    println!("{}", increase);
}
