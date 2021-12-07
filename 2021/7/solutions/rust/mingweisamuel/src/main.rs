use std::io::BufRead;

fn triangular(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn dist(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() {
    let line = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .next()
        .expect("Empty input");

    let mut max = 0;
    let positions: Vec<usize> = line
        .split(',')
        .map(|num_str| num_str.parse::<usize>().expect("Failed to parse integer."))
        .inspect(|pos| {
            max = std::cmp::max(max, *pos);
        })
        .collect();
    let len = max + 1;

    let cost_lin = (0..len)
        .map(|i| positions.iter().map(|&pos| dist(pos, i)).sum::<usize>())
        .min()
        .unwrap();

    let cost_tri = (0..len)
        .map(|i| {
            positions
                .iter()
                .map(|&pos| triangular(dist(pos, i)))
                .sum::<usize>()
        })
        .min()
        .unwrap();

    println!("{}\n{}", cost_lin, cost_tri);
}
