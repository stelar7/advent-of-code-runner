use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;

const START: (isize, isize) = (0, 500);

pub fn main() {
    let rocks: HashSet<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (x, y) = pair.split_once(',').unwrap();
                    (y.parse::<isize>().unwrap(), x.parse::<isize>().unwrap())
                })
                .tuple_windows()
                .collect::<Vec<_>>()
        })
        .flat_map(|((ay, ax), (by, bx))| {
            let y_min = std::cmp::min(ay, by);
            let y_max = std::cmp::max(ay, by);
            let x_min = std::cmp::min(ax, bx);
            let x_max = std::cmp::max(ax, bx);
            let col = (y_min..=y_max).map(move |y| (y, x_max));
            let row = (x_min..=x_max).map(move |x| (y_max, x));
            col.chain(row)
        })
        .collect();

    let mut p1 = 0;
    let void = rocks.iter().map(|&(y, _x)| y).max().unwrap();
    let mut sands = rocks.clone();
    while !sands.contains(&START) {
        let mut sand = START;
        'gravity: loop {
            if 0 == p1 && void <= sand.0 {
                // Sand fallen into the "void"
                p1 = sands.len() - rocks.len();
            }
            if sand.0 <= void {
                for dx in [0, -1, 1] {
                    let next = (sand.0 + 1, sand.1 + dx);
                    if !sands.contains(&next) {
                        sand = next;
                        continue 'gravity;
                    }
                }
            }
            sands.insert(sand);
            break;
        }
    }

    #[cfg(debug_assertions)]
    {
        for y in -1..25 {
            for x in 475..=525 {
                print!(
                    "{}",
                    if rocks.contains(&(y, x)) {
                        '#'
                    } else if sands.contains(&(y, x)) {
                        'o'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    let p2 = sands.len() - rocks.len();
    println!("{}\n{}", p1, p2);
}
