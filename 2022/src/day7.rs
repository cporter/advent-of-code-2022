#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::cmp::min;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Directory {
    subdirs : HashMap<String, Directory>,
    files : HashMap<String, usize>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
    fn add_dir(&mut self, dirname : String) {
        let d = Directory { subdirs: HashMap::new(), files: HashMap::new() };
        self.subdirs.insert(dirname, d);

    }
    fn add_file(&mut self, filename : String, filesize: usize) {
        self.files.insert(filename, filesize);
    }

    fn consume(&mut self, iter: &mut dyn Iterator<Item = LineType>) {
        loop {
            let nxt = iter.next();
            match nxt {
                None => { break; },
                Some(cmd) => match cmd {
                    LineType::Ls => { },
                    LineType::File(filename, filesize) => self.add_file(filename, filesize),
                    LineType::Dir(dirname) => self.add_dir(dirname),
                    LineType::Cd(dirname) => {
                        if dirname == ".." {
                            break;
                        } else if dirname != "/" {
                            let subdir = self.subdirs.get_mut(&dirname).unwrap();
                            subdir.consume(iter);
                        }
                    }
                }
            }
        }
    }

    fn dirsize(&self) -> usize {
        self.files.values().sum::<usize>() +
        self.subdirs.values().map(|d| d.dirsize()).sum::<usize>()
    }

    fn day1(&self) -> usize {
        let ds = self.dirsize();
        return self.subdirs.values().map(|d| d.day1()).sum::<usize>() + if ds <= 100000 {
            ds
        } else {
            0
        }
    }

    fn day2(&self, needed : usize, smallest : usize) -> usize {
        let ds = self.dirsize();
        let ret = if ds >= needed {
            min(ds, smallest)
        } else {
            smallest
        };
        let subdirs = self.subdirs.values()
            .map(|d| d.day2(needed, ret));
        subdirs.reduce(min).unwrap_or(ret)
    }

    fn total(&self) -> usize {
        return self.files.values().sum::<usize>() +
            self.subdirs.values().map(|d| d.total()).sum::<usize>();
    }
}

#[derive(Debug)]
enum LineType {
    Cd(String),
    Ls,
    Dir(String),
    File(String, usize),
}

impl FromStr for LineType {
    type Err = String;
    fn from_str(line: &str) -> Result<LineType, Self::Err> {
        lazy_static! {
            static ref CD_REGEX: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
            static ref LS_REGEX: Regex = Regex::new(r"^\$ ls$").unwrap();
            static ref DIR_REGEX: Regex = Regex::new(r"^dir (.+)").unwrap();
            static ref FILE_REGEX: Regex = Regex::new(r"^(\d+)\s+(.+)$").unwrap();
        }
        match CD_REGEX.captures(&line) {
            None => {},
            Some(m) => {
                return Ok(LineType::Cd(m[1].to_string()));
            },
        }
        match LS_REGEX.captures(&line) {
            None => {},
            Some(_) => return Ok(LineType::Ls),
        }
        match DIR_REGEX.captures(&line) {
            None => {},
            Some(m) => return Ok(LineType::Dir(m[1].to_string())),
        }
        match FILE_REGEX.captures(&line) {
            None => {},
            Some(m) => return Ok(LineType::File(m[2].to_string(), m[1].parse().unwrap())),
        }

        panic!("Unexpected input: {}", line);
    }
}


fn main() {
    let mut root : Directory = Directory::new();

    let mut iter = io::stdin().lock().lines()
        .map(|s| s.unwrap().parse::<LineType>().unwrap());

    root.consume(&mut iter);

    let day1 = root.day1();
    let system: usize = 70000000;
    let required: usize = 30000000;
    let total = root.total();
    let avail = system - total;
    let to_free = required - avail;
    
    let day2 = root.day2(to_free, system);


    println!("{}", day1);
    println!("{}", day2);
}