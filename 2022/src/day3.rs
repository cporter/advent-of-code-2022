use std::collections::HashSet;
use std::io::{self, BufRead};

fn priority(ch : char) -> i32 {
    if ch >= 'a' && ch <= 'z' {
        return 1 + (ch as i32) - ('a' as i32);
    } else if ch >= 'A' && ch <= 'Z' {
        return 27 + (ch as i32) - ('A' as i32);
    } else {
        panic!("Illegal character: {}", ch);
    }
}

#[derive(Debug)]
struct Rucksack {
    a : Vec<char>,
    b : Vec<char>,
}

fn shared_priorities(r : &Rucksack) -> i32 {
    return r.a.iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|c| r.b.contains(c))
        .map(|c| priority(*c))
        .sum();
}

fn from_line(s : &str) -> Rucksack {
    let chrs : Vec<char> = s.chars().collect();
    let size = chrs.len();
    let half = size / 2;
    let s = chrs.as_slice();
    return Rucksack {
        a: s[0..half].to_vec(),
        b: s[half..].to_vec(),
    };
}

fn letters(r : &Rucksack) -> HashSet<char> {
    return r.a.iter().map(|ch| *ch)
        .chain(r.b.iter().map(|ch| *ch))
        .collect::<HashSet<char>>();    
}

fn main() {
    let stdin = io::stdin();
    let rucksacks : Vec<Rucksack> = stdin
        .lock()
        .lines()
        .map(|s| from_line(&s.unwrap()))
        .collect();

    let part1 : i32 = rucksacks
        .iter()
        .map(|r| shared_priorities(&r))
        .sum();

    let part2 : i32 = (0..rucksacks.len()).step_by(3)
        .map(|i| {
            let la = letters(&rucksacks[i]);
            let lb = letters(&rucksacks[1+i]);
            let lc = letters(&rucksacks[2+i]);
            let ab = la.intersection(&lb).map(|ch|*ch).collect::<HashSet<_>>();
            let abc = ab.intersection(&lc).map(|ch|*ch).collect::<HashSet<_>>();
            assert!(1 == abc.len());
            return *abc.iter().next().unwrap();
        })
        .map(|ch| priority(ch))
        .sum();

    
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}