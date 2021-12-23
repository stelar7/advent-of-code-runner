use scan_fmt::scan_fmt;
use std::io::BufRead;
use std::ops::RangeInclusive;

fn simulate(xb: &RangeInclusive<i32>, yb: &RangeInclusive<i32>, mut vx: i32, mut vy: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut y_max = 0;

    while *yb.start() <= y {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
        y_max = std::cmp::max(y_max, y);

        if xb.contains(&x) && yb.contains(&y) {
            return Some(y_max);
        }
    }
    None
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .next()
        .expect("Empty input.");

    let (xn, xx, yn, yx) = scan_fmt!(
        &*line,
        "target area: x={d}..{d}, y={d}..{d}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();

    let xb = xn..=xx;
    let yb = yn..=yx;

    let mut max_y_all = 0;
    let mut count = 0;
    for vx in 0..=xx {
        for vy in yn..=(-yn) {
            if let Some(max_y) = simulate(&xb, &yb, vx, vy) {
                max_y_all = std::cmp::max(max_y_all, max_y);
                count += 1;
            }
        }
    }
    println!("{}\n{}", max_y_all, count);
}
