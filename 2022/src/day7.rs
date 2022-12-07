#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct File {
    name : String,
    size : usize,
}

#[derive(Debug)]
struct Directory {
    subdirs : HashMap<String, Box<Directory>>,
    files : HashMap<String, usize>,
}

impl Directory {
    fn add_dir(&self, dirname : String) {
        let d = Directory { subdirs: HashMap::new(), files: HashMap::new() };
        let bd = Box::new(d);
        self.subdirs[&dirname] = bd;
    }
}

#[derive(Debug)]
enum LineType {
    Cd(String),
    CdUp, // reasonable to special case this one
    Ls,
    Dir(String),
    File(String, usize),
}

impl FromStr for LineType {
    type Err = String;
    fn from_str(line: &str) -> Result<LineType, Self::Err> {
        lazy_static! {
            static ref cdup_regex: Regex = Regex::new(r"^\$ cd \.\.$").unwrap();
            static ref cd_regex: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
            static ref ls_regex: Regex = Regex::new(r"^\$ ls$").unwrap();
            static ref dir_regex: Regex = Regex::new(r"^dir (.+)").unwrap();
            static ref fil_regex: Regex = Regex::new(r"^(\d+)\s+(.+)$").unwrap();
        }

        match cdup_regex.captures(&line) {
            None => {},
            Some(_) => return Ok(LineType::CdUp),
        }
        match cd_regex.captures(&line) {
            None => {},
            Some(m) => return Ok(LineType::Cd(m[1].to_string())),
        }
        match ls_regex.captures(&line) {
            None => {},
            Some(m) => return Ok(LineType::Ls),
        }
        match dir_regex.captures(&line) {
            None => {},
            Some(m) => return Ok(LineType::Dir(m[1].to_string())),
        }
        match fil_regex.captures(&line) {
            None => {},
            Some(m) => return Ok(LineType::File(m[2].to_string(), m[1].parse().unwrap())),
        }

        panic!("Unexpected input: {}", line);
    }
}


fn main() {
    let mut tree : Vec<Box<Directory>> = Vec::new();

    io::stdin().lock().lines()
        .map(|s| s.unwrap().parse::<LineType>().unwrap())
        .for_each(|l| {
            match l {
                LineType::Cd(dir) => {
                    if "/" == dir {
                        if 0 == tree.len() {
                            tree.push(Box::new(Directory {
                                subdirs: HashMap::new(),
                                files: HashMap::new(),
                            }));
                        } else {
                            while 1 < tree.len() {
                                tree.pop();
                            }
                        }
                    } else {
                        let current = tree.last().unwrap();
                        let next = current.subdirs.get(&dir).unwrap();
                        tree.push(*next);
                    }
                },
                LineType::CdUp => {
                    tree.pop();
                },
                LineType::Ls => {
                    // this is sort of a noop? Am I missing something?
                },
                LineType::Dir(dirname) => {
                    let current = tree.last().unwrap();
                    current.subdirs.insert(dirname, 
                        Box::new(Directory { files: HashMap::new(), subdirs: HashMap::new() }));                
                },
                LineType::File(filename, size) => {
                    let current = tree.last().unwrap();
                    current.files.insert(filename.to_string(), size);
                }
            }
        });
    println!("{:?}", tree[0]);
}