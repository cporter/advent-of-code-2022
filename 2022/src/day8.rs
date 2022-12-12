use std::io::BufRead;
use std::cmp::max;

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool,
}

type Trees = Vec<Vec<Tree>>;



fn scenic_score(t: &Trees, c: usize, r: usize) -> usize {
    let max_cols = t[0].len();
    let max_rows = t.len();
    let h = t[r][c].height;

    let mut lc: usize = 0;
    let mut rc: usize = 0;
    let mut uc: usize = 0;
    let mut dc: usize = 0;

    for cc in (0..c).rev() {
        lc += 1;
        if t[r][cc].height >= h {
            break;
        }
    }
    for cc in (1 + c)..max_cols {
        rc += 1;
        if t[r][cc].height >= h {
            break;
        }
    }
    for rr in (0..r).rev() {
        uc += 1;
        if t[rr][c].height >= h {
            break;
        }
    }
    for rr in (1 + r)..max_rows {
        dc += 1;
        if t[rr][c].height >= h {
            break;
        }
    }
    lc * rc * uc * dc
}

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

    let mut day2 = 0;
    for r in 0..rows {
        for c in 0..cols {
            day2 = max(day2, scenic_score(&trees, r, c));
        }
    }

    println!("day1: {}", day1);
    println!("day2: {}", day2);
}
