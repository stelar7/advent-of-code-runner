use itertools::Itertools;
use std::cmp::Reverse;
use std::io::BufRead;

fn main() {
    const N: usize = 3;
    let result: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Failed to read line as UTF-8.")
                .parse::<usize>()
                .ok()
        })
        .coalesce(|a, b| match (a, b) {
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (None, Some(b)) => Ok(Some(b)),
            other => Err(other),
        })
        .map(Option::unwrap)
        .map(Reverse)
        .k_smallest(N)
        .collect();
    if let [Reverse(a), Reverse(b), Reverse(c)] = result[..] {
        println!("{}\n{}", a, a + b + c);
    } else {
        panic!();
    }
}
