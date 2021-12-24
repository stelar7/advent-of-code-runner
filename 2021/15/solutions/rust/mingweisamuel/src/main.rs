use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::BufRead;

fn pathfind_dist(grid: &[Vec<u8>], bound: usize) -> usize {
    let size = grid.len();

    let mut dist: HashMap<(usize, usize), usize> = Default::default();
    let mut queue = PriorityQueue::new();
    queue.push((0, 0), Reverse(0));
    dist.insert((0, 0), 0);

    let mut neighbors = Vec::with_capacity(4);
    while let Some(((r, c), Reverse(_dist_est))) = queue.pop() {
        if (bound, bound) == (r, c) {
            break;
        }

        if 0 < r {
            neighbors.push((r - 1, c));
        }
        if r < bound {
            neighbors.push((r + 1, c));
        }
        if 0 < c {
            neighbors.push((r, c - 1));
        }
        if c < bound {
            neighbors.push((r, c + 1));
        }

        let d = dist[&(r, c)];
        for (nr, nc) in neighbors.drain(..) {
            let edge_cost = {
                let (rq, rr) = (nr / size, nr % size);
                let (cq, cr) = (nc / size, nc % size);
                ((grid[rr][cr] as usize) + rq + cq - 1) % 9 + 1
            };
            let alt = d + edge_cost;
            if dist
                .get(&(nr, nc))
                .map(|&old_dist| alt < old_dist)
                .unwrap_or(true)
            {
                dist.insert((nr, nc), alt);
                queue.push((nr, nc), Reverse(alt + 2 * bound - nr - nc));
            }
        }
    }
    *dist.get(&(bound, bound)).unwrap()
}

fn main() {
    let stdin = std::io::stdin();
    let grid: Vec<Vec<u8>> = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| line.into_bytes())
        .map(|mut row| {
            for b in row.iter_mut() {
                *b -= b'0';
            }
            row
        })
        .collect();

    let size = grid.len();
    assert_eq!(size, grid[0].len());

    let bound = 5 * size - 1;

    let part_a = pathfind_dist(&*grid, size - 1);
    let part_b = pathfind_dist(&*grid, bound);

    println!("{}\n{}", part_a, part_b);
}
