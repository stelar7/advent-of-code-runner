use std::collections::HashMap;
use std::io::BufRead;

pub fn range(a: isize, b: isize) -> std::ops::RangeInclusive<isize> {
    std::cmp::min(a, b)..=std::cmp::max(a, b)
}

fn main() {
    let mut counts_orth: HashMap<(isize, isize), isize> = Default::default();
    let mut counts_both: HashMap<(isize, isize), isize> = Default::default();

    let stdin = std::io::stdin();
    for (src_x, src_y, dst_x, dst_y) in stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let (src, dst) = line.split_once(" -> ").unwrap();
            let (src_x, src_y) = src.split_once(',').unwrap();
            let (dst_x, dst_y) = dst.split_once(',').unwrap();
            let src_x: isize = src_x.parse().unwrap();
            let src_y: isize = src_y.parse().unwrap();
            let dst_x: isize = dst_x.parse().unwrap();
            let dst_y: isize = dst_y.parse().unwrap();
            (src_x, src_y, dst_x, dst_y)
        })
    {
        let mut x = src_x;
        let mut y = src_y;
        let dx = (dst_x - src_x).signum();
        let dy = (dst_y - src_y).signum();
        let orth = 0 == dx || 0 == dy;

        while !(x == dst_x && y == dst_y) {
            if orth {
                *counts_orth.entry((x, y)).or_default() += 1;
            }
            *counts_both.entry((x, y)).or_default() += 1;

            x += dx;
            y += dy;
        }

        if orth {
            *counts_orth.entry((x, y)).or_default() += 1;
        }
        *counts_both.entry((x, y)).or_default() += 1;
    }

    let doubles_orth = counts_orth.values().filter(|count| 2 <= **count).count();
    let doubles_both = counts_both.values().filter(|count| 2 <= **count).count();

    println!("{}\n{}", doubles_orth, doubles_both);
}
