use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

const ORTHO: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .enumerate()
        .map(|(r, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(c, b)| match b {
                    b'S' => {
                        start = (r as isize, c as isize);
                        0
                    }
                    b'E' => {
                        end = (r as isize, c as isize);
                        b'z' - b'a'
                    }
                    b => b - b'a',
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let mut p1 = 0;
    let mut p2 = 0;

    let mut seen = HashSet::new();
    let mut next = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    while let Some((curr, curr_dist)) = queue.pop_front() {
        if start == curr {
            p1 = curr_dist;
            break;
        }
        if seen.insert(curr) {
            let curr_height = grid[curr.0 as usize][curr.1 as usize];
            if 0 == p2 && 0 == curr_height {
                p2 = curr_dist;
            }

            let neighbors = ORTHO
                .iter()
                .map(|&(d0, d1)| ((curr.0 as isize) + d0, (curr.1 as isize) + d1))
                .filter(|(n0, n1)| (0..h).contains(&n0) && (0..w).contains(&n1))
                .filter(|&(n0, n1)| {
                    let n_height = grid[n0 as usize][n1 as usize];
                    curr_height.saturating_sub(n_height) <= 1
                })
                .filter(|neighbor| !seen.contains(neighbor))
                .inspect(|&neighbor| {
                    next.insert(neighbor, curr);
                })
                .map(|neighbor| (neighbor, curr_dist + 1));
            queue.extend(neighbors);
        }
    }

    #[cfg(debug_assertions)]
    {
        let mut path_draw = HashMap::new();
        let mut curr = start;
        while end != curr {
            let next = next[&curr];
            match (next.0 - curr.0, next.1 - curr.1) {
                (-1, 0) => path_draw.insert(next, '^'),
                (0, -1) => path_draw.insert(next, '<'),
                (0, 1) => path_draw.insert(next, '>'),
                (1, 0) => path_draw.insert(next, 'v'),
                _ => panic!(),
            };
            curr = next;
        }
        for r in 0..h {
            for c in 0..w {
                let c = if let Some(&c) = path_draw.get(&(r, c)) {
                    c
                } else if seen.contains(&(r, c)) {
                    '.'
                } else {
                    ' '
                };
                print!("{}", c);
            }
            println!();
        }
    }

    debug_assert_ne!(0, p1);
    debug_assert_ne!(0, p2);
    println!("{}\n{}", p1, p2);
}
