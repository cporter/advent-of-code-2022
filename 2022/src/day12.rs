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

fn bfs<F>(coords: &mut Vec<Coord>, start_name: char, test: F)
where
    F: Fn(&Coord, &Coord) -> bool,
{
    let steps: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let ncols = coords.iter().map(|c| c.col).max().unwrap() + 1;
    let nrows = coords.iter().map(|c| c.row).max().unwrap() + 1;

    let start = coords
        .iter()
        .filter(|c| c.name == start_name)
        .next()
        .unwrap();
    q.push_front((start.row, start.col));

    while !q.is_empty() {
        let (r, c) = q.pop_back().unwrap();
        let ind = (r * ncols + c) as usize;
        steps.iter().for_each(|(x, y)| {
            let rr = r + x;
            let cc = c + y;
            if rr >= 0 && cc >= 0 && cc < ncols && rr < nrows {
                let next_ind = (rr * ncols + cc) as usize;
                if test(&coords[ind], &coords[next_ind]) {
                    if 0 == coords[next_ind].distance {
                        coords[next_ind].distance = 1 + coords[ind].distance;
                        q.push_front((coords[next_ind].row, coords[next_ind].col));
                    }
                }
            }
        });
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

    bfs(&mut coords, 'S', |from, to| 1 + from.height >= to.height);

    let part1 = coords
        .iter()
        .filter(|c| c.name == 'E')
        .next()
        .unwrap()
        .distance;
    println!("part 1: {part1}");

    for coord in coords.iter_mut() {
        coord.distance = 0;
    }

    bfs(&mut coords, 'E', |from, to| from.height <= 1 + to.height);

    let part2 = coords
        .iter()
        .filter(|c| c.name == 'a' || c.name == 'S')
        .map(|c| c.distance)
        .filter(|d| *d > 0)
        .min()
        .unwrap();

    println!("part 2: {part2}"); // 607 is too high

}
