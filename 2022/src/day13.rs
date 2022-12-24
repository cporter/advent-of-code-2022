use std::io::BufRead;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Int(i32),
    List(Vec<Cell>),
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Cell) -> Ordering {
        use Cell::*;
        match (self, other) {
            (Int(l), Int(r)) => l.cmp(r),
            (Int(l), List(_)) => List(vec![Int(*l)]).cmp(other),
            (List(_), Int(r)) => self.cmp(&List(vec![Int(*r)])),
            (List(l), List(r)) => {
                for (ll, rr) in l.iter().zip(r.iter()) {
                    let result = ll.cmp(rr);
                    let Ordering::Equal = result else {
                        return result
                    };
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

fn proper_ordered(lhs: &Cell, rhs: &Cell) -> bool {
    lhs < rhs
}

#[derive(Debug)]
enum Token {
    LP,
    RP,
    Int(i32),
    Comma
}

struct Tokens {
    ind: usize,
    data: Vec<char>,
}

impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ind >= self.data.len() {
            None
        } else {
            let ch = self.data[self.ind];
            match ch {
                '[' => { self.ind += 1; Some(Token::LP) },
                ']' => { self.ind += 1; Some(Token::RP) },
                ',' => { self.ind += 1; Some(Token::Comma) },
                _ => { // we'll assume that everything here is a number
                    let mut num : Vec<char> = vec![];
                    while self.data[self.ind].is_digit(10) {
                        num.push(self.data[self.ind]);
                        self.ind += 1;
                    }
                    let s : String = num.iter().collect();
                    return Some(Token::Int(s.parse().unwrap()))
                }
            }
        }
    }
}


fn parse_cell_list<I>(iter: &mut I) -> Cell
where
    I: Iterator<Item = Token>,
{
    let mut ret: Vec<Cell> = Vec::new();
    let mut quit = false;
    while !quit {
        match iter.next() {
            None => panic!("Malformed input"),
            Some(ch) => {
                match ch {
                    Token::LP => ret.push(parse_cell_list(iter)),
                    Token::RP => {
                        quit = true;
                    }
                    Token::Int(n) => ret.push(Cell::Int(n)),
                    Token::Comma => {}
                }
            }
        }
    }
    Cell::List(ret)
}

fn parse_cell_inner<I>(iter: &mut I) -> Cell
where
    I: Iterator<Item = Token>,
{
    match iter.next() {
        None => panic!("malformed input!"),
        Some(Token::LP) => parse_cell_list(iter),
        Some(Token::Int(n)) => Cell::Int(n),
        Some(x) => panic!("Unexpected input: {:?}", x)
    }
}

fn parse_cell(line: String) -> Cell {
    let mut iter = Tokens {
        ind: 0,
        data: line.chars().collect(),
    };

    parse_cell_inner(&mut iter)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day13_sample_2() {
        let left = parse_cell("[[1],[2,3,4]]".to_string());
        let right = parse_cell("[[1],4]".to_string());
        assert_eq!(true, proper_ordered(&left, &right));        
    }

    #[test]
    fn test_day13_sample_4() {
        let left = parse_cell("[[4,4],4,4]".to_string());
        let right = parse_cell("[[4,4],4,4,4]".to_string());
        assert_eq!(true, proper_ordered(&left, &right));
    }

    #[test]
    fn test_day13_input_6() {
        let left = parse_cell("[[[7,3,[]],0],[3,[2]]]".to_string());
        let right = parse_cell("[[[[10,1,7],[7]]],[[7,6,7]],[[3,[6,4],[9,9,5],[2,8,5,4]],10],[[[4,5,10,1,1],[4,10],[8,9],[6,8,1,6],9],[[8,9],4,[],1,1],[2,[6,0],10,[],8],2],[8,[[],2]]]".to_string());
        assert_eq!(true, proper_ordered(&left, &right));
    }
}

fn main() {
    let cells: Vec<Cell> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| 0 < line.len())
        .map(|line| parse_cell(line))
        .collect();

    let part1: usize = (0..cells.len())
        .step_by(2)
        .filter(|i| cells[*i] < cells[i+1])
        .map(|i| 1 + i / 2)
        .sum();

    println!("part 1: {part1}"); // 5729 is too high
}
