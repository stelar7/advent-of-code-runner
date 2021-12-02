use std::io::BufRead;
use itertools::Itertools;

fn main() {
    let mut bumps_1 = 0;
    let mut bumps_3 = 0;

    std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Failed to read line.")
                .parse::<usize>()
                .expect("Failed to parse int.")
        })
        .scan(usize::MAX, |prev, curr| {
            if *prev < curr {
                bumps_1 += 1;
            }
            *prev = curr;
            Some(curr)
        })
        .tuple_windows()
        .scan(usize::MAX, |prev, (a, b, c)| {
            let curr = a + b + c;
            if *prev < curr {
                bumps_3 += 1;
            }
            *prev = curr;
            Some(())
        })
        .for_each(std::mem::drop);

    println!("{}\n{}", bumps_1, bumps_3);
}
