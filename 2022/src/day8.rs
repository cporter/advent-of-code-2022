use std::io::BufRead;


#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool,
}

type Trees = Vec<Vec<Tree>>;

fn main() {
    let mut trees: Trees = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| {
                    Tree {
                        height: ch.to_digit(10).unwrap() as i8,
                        visible: false,
                    }
                })
                .collect()
        })
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();

    for row in 0..rows {
        let mut biggest: i8 = -1;
        for col in 0..cols {
            let mut tree = &mut trees[row][col];
            if tree.height > biggest {
                tree.visible = true;
                biggest = tree.height;
            }
        }
        biggest = -1;
        for col in (0..cols).rev() {
            let mut tree = &mut trees[row][col];
            if tree.height > biggest {
                tree.visible = true;
                biggest = tree.height;
            }
        }
    }

    for col in 0..cols {
        let mut biggest: i8 = -1;
        for row in 0..rows {
            let mut tree = &mut trees[row][col];
            if tree.height > biggest {
                tree.visible = true;
                biggest = tree.height;
            }
        }
        biggest = -1;
        for row in (0..rows).rev() {
            let mut tree = &mut trees[row][col];
            if tree.height > biggest {
                tree.visible = true;
                biggest = tree.height;
            }
        }
    }

    let day1: usize = trees
        .iter()
        .map(|row| row.iter().filter(|t| t.visible).count())
        .sum();

    println!("day1: {}", day1);
}
