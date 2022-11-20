use std::io::{self, BufRead};
use std::iter::zip;

fn main() {
    let stdin = io::stdin();
    let v : Vec<i32> = stdin
	.lock()
	.lines()
	.filter_map(|s| s.unwrap().parse().ok())
	.collect();

    let a = v.iter();
    let b = v.iter().skip(1);
    let c = v.iter().skip(2);

    let sliding : Vec<i32> = zip(a, zip(b, c))
	.map(|(i, (j, k))| i + j + k)
	.collect();

    let head = sliding.iter();
    let tail = sliding.iter().skip(1);

    let increase = zip(head, tail)
	.filter(|(prev, succ)| prev < succ)
	.count();

    println!("{}", increase);
}
