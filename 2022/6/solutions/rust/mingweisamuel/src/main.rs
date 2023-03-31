use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let line = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("Expected one line of input.")
        .expect("Failed to read line as UTF-8.");

    let row = line.as_bytes();
    println!("{}\n{}", unique_run_end(row, 4), unique_run_end(row, 14));
}

fn unique_run_end(row: &[u8], len: usize) -> usize {
    let mut counts = row[0..len - 1].iter().copied().counts();
    for (i, (&old, &new)) in row.iter().zip(&row[len - 1..]).enumerate() {
        *counts.entry(new).or_default() += 1;
        // println!("{:#?}", counts.iter().map(|(&c, &count)| (c as char, count)).collect::<std::collections::HashMap<_, _>>());
        if counts.values().all(|&n| n <= 1) {
            return i + len;
        }
        *counts.entry(old).or_default() -= 1;
    }
    panic!();
}
