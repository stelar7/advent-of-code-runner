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
    let mut duped_chars = counts.values().filter(|&&n| 1 < n).count();
    for (i, (&old, &new)) in row.iter().zip(&row[len - 1..]).enumerate() {
        let count_new = counts.entry(new).or_default();
        *count_new += 1;
        if 2 == *count_new {
            duped_chars += 1;
        }

        // println!("{:#?}", counts.iter().map(|(&c, &count)| (c as char, count)).collect::<std::collections::HashMap<_, _>>());
        if 0 == duped_chars {
            return i + len;
        }

        let count_old = counts.entry(old).or_default();
        *count_old -= 1;
        if 1 == *count_old {
            duped_chars -= 1;
        }
    }
    panic!();
}
