use std::io::BufRead;
use std::collections::VecDeque;

#[derive(Debug)]
struct Coord {
    row: i32,
    col: i32,
    height: i32,
    name: char,
    distance: i32,
}

fn height(ch: char) -> i32 {
    match ch {
        'S' => 0,
        'E' => 25,
        _ => (ch as i32) - ('a' as i32),
    }
}



fn main() {
    let steps: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut coords: Vec<Coord> = std::io::stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, ch)| {
                    Coord {
                        row: row as i32,
                        col: col as i32,
                        height: height(ch),
                        name: ch,
                        distance: 0,
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .flatten()
        .collect();

    let begin = coords.iter().filter(|c| c.name == 'S').next().unwrap();

    q.push_front((begin.row, begin.col));
    let ncols = coords.iter().map(|c| c.col).max().unwrap() + 1;
    let nrows = coords.iter().map(|c| c.row).max().unwrap() + 1;

    while !q.is_empty() {
        let (r, c) = q.pop_back().unwrap();
        let ind = (r * ncols + c) as usize;
        steps.iter().for_each(|(x, y)| {
            let rr = r + x;
            let cc = c + y;
            if rr >= 0 && cc >= 0 && cc < ncols && rr < nrows {
                let next_ind = (rr * ncols + cc) as usize;
                if (coords[next_ind].height - 1) <= coords[ind].height {
                    if 0 == coords[next_ind].distance {
                        coords[next_ind].distance = 1 + coords[ind].distance;
                        q.push_front((coords[next_ind].row, coords[next_ind].col));
                    }
                }
            }
        });
    }

    {
        let end = coords.iter().filter(|c| c.name == 'E').next().unwrap();
        println!("part 1: {}", end.distance);
    }

    for coord in coords.iter_mut() {
        coord.distance = 0;
    }

    let end = coords.iter().filter(|c| c.name == 'E').next().unwrap();
    q.push_front((end.row, end.col));
    while !q.is_empty() {
        let (r, c) = q.pop_back().unwrap();
        let ind = (r * ncols + c) as usize;
        steps.iter().for_each(|(x, y)| {
            let rr = r + x;
            let cc = c + y;
            if rr >= 0 && cc >= 0 && cc < ncols && rr < nrows {
                let next_ind = (rr * ncols + cc) as usize;
                if (coords[next_ind].height + 1) >= coords[ind].height {
                    if 0 == coords[next_ind].distance {
                        coords[next_ind].distance = 1 + coords[ind].distance;
                        q.push_front((coords[next_ind].row, coords[next_ind].col));
                    }
                }
            }
        });
    }

    let part2 = coords
        .iter()
        .filter(|c| c.name == 'a' || c.name == 'S')
        .map(|c| c.distance)
        .filter(|d| *d > 0)
        .min()
        .unwrap();

    println!("part 2: {part2}"); // 607 is too high

}
