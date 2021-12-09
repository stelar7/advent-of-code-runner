use std::collections::HashMap;
use std::io::BufRead;

fn disjoint_set_get<K: Copy + Eq + std::hash::Hash>(edges: &mut HashMap<K, K>, k: K) -> K {
    if let Some(&k_next) = edges.get(&k) {
        let k_last = disjoint_set_get(edges, k_next);
        edges.insert(k, k_last);
        k_last
    } else {
        k
    }
}

fn main() {
    let grid = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut low_points = HashMap::new();
    let mut targets: HashMap<_, _> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (y, x)))
        .filter_map(|(y, x)| {
            let v = grid[y][x];
            if 9 <= v {
                return None;
            }

            let n = y.checked_sub(1).map(|y1| grid[y1][x]).unwrap_or(u8::MAX);
            let s = grid.get(y + 1).map(|row| row[x]).unwrap_or(u8::MAX);
            let w = x.checked_sub(1).map(|x1| grid[y][x1]).unwrap_or(u8::MAX);
            let e = grid[y].get(x + 1).copied().unwrap_or(u8::MAX);

            let min = [v, n, s, w, e].into_iter().min().unwrap();
            if min == v {
                low_points.insert((y, x), 0);
                None
            } else if min == n {
                Some(((y, x), (y - 1, x)))
            } else if min == s {
                Some(((y, x), (y + 1, x)))
            } else if min == w {
                Some(((y, x), (y, x - 1)))
            } else if min == e {
                Some(((y, x), (y, x + 1)))
            } else {
                unreachable!();
            }
        })
        .collect();

    for (y, x) in (0..height).flat_map(|y| (0..width).map(move |x| (y, x))) {
        let v = grid[y][x];
        if 9 == v {
            continue;
        }

        let target = disjoint_set_get(&mut targets, (y, x));
        *low_points.get_mut(&target).unwrap() += 1;
    }

    let p1: usize = low_points
        .keys()
        .map(|&(y, x)| (grid[y][x] as usize) + 1)
        .sum();

    let mut heap = low_points
        .values()
        .collect::<std::collections::BinaryHeap<_>>();
    let p2 = heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap();

    println!("{}\n{}", p1, p2);
}
