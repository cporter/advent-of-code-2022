#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
enum Value {
    Old,
    Num(i64),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    op_a: Value,
    op_b: Value,
    test: i64,
    true_throw: usize,
    false_throw: usize,
    counted: usize,
}

fn inspect(m: &Monkey, x: i64) -> i64 {
    let l = match m.op_a {
        Value::Old => x,
        Value::Num(n) => n,
    };
    let r = match m.op_b {
        Value::Old => x,
        Value::Num(n) => n,
    };
    match m.op {
        Op::Add => l + r,
        Op::Sub => l - r,
        // Op::Mul => l * r,
        Op::Mul => {
            match l.checked_mul(r) {
                Some(x) => x,
                None => panic!("still trying to multiply {l} and {r}"),
            }
        }
    }
}

impl Default for Monkey {
    fn default() -> Monkey {
        Monkey {
            items: vec![],
            op: Op::Add,
            op_a: Value::Old,
            op_b: Value::Old,
            test: 0,
            true_throw: 0,
            false_throw: 0,
            counted: 0,
        }
    }
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(lines_raw: &str) -> Result<Monkey, Self::Err> {
        lazy_static! {
            static ref ITEMS: Regex = Regex::new(r": (.+)$").unwrap();
            static ref OPERATION: Regex = Regex::new(r"new = ([^\s]+) ([-+*]) ([^\s]+)").unwrap();
            static ref TEST: Regex = Regex::new(r"divisible by (\d+)").unwrap();
            static ref MONKEY: Regex = Regex::new(r"monkey (\d+)").unwrap();
        }
        let lines: Vec<&str> = lines_raw.split("\n").collect();
        let mut ret = Monkey::default();
        {
            let cap = ITEMS.captures_iter(lines[1]).next().unwrap();
            ret.items = cap[1].split(", ").map(|s| s.parse().unwrap()).collect();
        }
        {
            let cap = OPERATION.captures_iter(lines[2]).next().unwrap();
            ret.op = match &cap[2] {
                "+" => Op::Add,
                "*" => Op::Mul,
                "-" => Op::Sub,
                _ => panic!("Unexpected operator: {}", &cap[2]),
            };
            ret.op_a = match &cap[1] {
                "old" => Value::Old,
                _ => Value::Num(cap[1].parse().unwrap()),
            };
            ret.op_b = match &cap[3] {
                "old" => Value::Old,
                _ => Value::Num(cap[3].parse().unwrap()),
            };
        }
        {
            let cap = TEST.captures_iter(lines[3]).next().unwrap();
            ret.test = cap[1].parse().unwrap();
        }
        {
            let cap = MONKEY.captures_iter(lines[4]).next().unwrap();
            ret.true_throw = cap[1].parse().unwrap();
        }
        {
            let cap = MONKEY.captures_iter(lines[5]).next().unwrap();
            ret.false_throw = cap[1].parse().unwrap();
        }

        Ok(ret)
    }
}

fn solve(n: usize, divisor: i64, monkeys: &mut Vec<Monkey>) -> usize {
    let common_divisor: i64 = monkeys.iter().map(|m| m.test).product();
    (0..n).for_each(|_| for i in 0..monkeys.len() {
        monkeys[i].counted += monkeys[i].items.len();
        let items: Vec<i64> = monkeys[i].items.to_vec();
        monkeys[i].items.clear();

        items.iter().for_each(|item| {
            let val = *item % common_divisor;
            let newval = inspect(&monkeys[i], val) / divisor;
            let target = if 0 == newval % monkeys[i].test {
                monkeys[i].true_throw
            } else {
                monkeys[i].false_throw
            };
            monkeys[target].items.push(newval);
        });
    });

    let mut counts: Vec<usize> = monkeys.iter().map(|m| m.counted).collect();
    counts.sort();

    let s = &counts[counts.len() - 2..counts.len()];
    s.iter().map(|x| *x).product()
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut monkeys: Vec<Monkey> = stdin.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let part1 = solve(20, 3, &mut monkeys);

    monkeys = stdin.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let part2 = solve(10000, 1, &mut monkeys);

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
