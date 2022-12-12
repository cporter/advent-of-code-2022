use std::collections::HashMap;
use std::cmp::{PartialEq, PartialOrd};

#[derive(Debug)]
struct Tree<T: PartialOrd> {
    payload: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T: PartialOrd> Tree<T> {
    fn contains(self, t: T) -> bool {
        if t == self.payload {
            true
        } else if t < self.payload {
            match self.left {
                Some(tt) => tt.contains(t),
                None => false,
            }
        } else {
            match self.right {
                Some(tt) => tt.contains(t),
                None => false,
            }
        }
    }

    fn add(mut self, t: T) {
        if t < self.payload {
            match self.left {
                None => {
                    self.left = Some(Box::new(Tree {
                        payload: t,
                        left: None,
                        right: None,
                    }))
                }
                Some(tr) => tr.add(t),
            }
        } else if t > self.payload {
            match self.right {
                None => {
                    self.right = Some(Box::new(Tree {
                        payload: t,
                        left: None,
                        right: None,
                    }))
                }
                Some(tr) => tr.add(t),
            }
        }
    }
}

type IntTree = Tree<i32>;

fn main() {
    let mut t: Box<IntTree> = Box::new(IntTree {
        payload: 10,
        left: None,
        right: None,
    });
    println!("{:?}", t);
}
