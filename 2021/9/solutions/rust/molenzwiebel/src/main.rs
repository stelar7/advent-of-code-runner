use itertools::Itertools;
use std::ops::Mul;
use std::{collections::HashMap, io::BufRead};

fn run(entries: Vec<Vec<u8>>) -> (usize, usize) {
    let mut depth = 0;

    // assume a 100*100 = 10,000 sized grid
    let to_idx = |i: usize, j: usize| i * 100 + j;

    let mut flow_direction = [None::<usize>; 10000];
    let mut basin_sizes = HashMap::with_capacity(10000);

    // a low point is a point that is lower than all adjacent
    // numbers in the square grid represented by entries
    for (i, row) in entries.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            let up = i.checked_sub(1).map(|x| entries[x][j]).unwrap_or(u8::MAX);
            let down = entries.get(i + 1).map(|x| x[j]).unwrap_or(u8::MAX);
            let left = j.checked_sub(1).map(|x| row[x]).unwrap_or(u8::MAX);
            let right = *row.get(j + 1).unwrap_or(&u8::MAX);

            let min_value = *[up, down, left, right, col].iter().min().unwrap();

            if col < up && col < down && col < left && col < right {
                // this is the lowest point, everything flows to here
                depth += (col - b'0') as usize + 1;
            }

            if min_value == up {
                // flows up
                flow_direction[to_idx(i, j)] = Some(to_idx(i - 1, j));
            } else if min_value == down {
                // flows down
                flow_direction[to_idx(i, j)] = Some(to_idx(i + 1, j));
            } else if min_value == left {
                // flows left
                flow_direction[to_idx(i, j)] = Some(to_idx(i, j - 1));
            } else if min_value == right {
                // flows right
                flow_direction[to_idx(i, j)] = Some(to_idx(i, j + 1));
            }
        }
    }

    // Recursively look up where (i, j) flows to.
    // Update the lookup to immediately go there on future lookups
    let mut lookup = |i: usize, j: usize| {
        let mut idx = to_idx(i, j);
        let mut val = flow_direction[idx];

        loop {
            if let None = val {
                return idx;
            }

            let old_idx = idx;
            idx = val.unwrap();
            val = flow_direction[idx];

            if let Some(_) = val {
                flow_direction[old_idx] = val;
            }
        }
    };

    // compute basin sizes by finding the final point for each entry
    // and incrementing the counter for that entry
    for (i, row) in entries.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == b'9' {
                continue;
            }

            let target = lookup(i, j);
            *basin_sizes.entry(target).or_insert(0) += 1;
        }
    }

    let largest_three = basin_sizes
        .values()
        .sorted_by_key(|&&x| -x)
        .take(3)
        .copied()
        .reduce(i32::mul)
        .unwrap() as usize;

    (depth, largest_three)
}

fn main() {
    let entries = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (a, b) = run(entries);
    println!("{}\n{}", a, b);
}
