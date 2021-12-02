use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let mut bumps_1 = 0;
    let mut bumps_3 = 0;

    std::io::stdin()
        .lock()
        .lines()
        .map(|line| lexical_core::parse(line.unwrap().as_bytes()).unwrap())
        .scan(u16::MAX, |prev, curr| {
            if *prev < curr {
                bumps_1 += 1;
            }
            *prev = curr;
            Some(curr)
        })
        .tuple_windows()
        .fold(u16::MAX, |prev, (a, b, c)| {
            let curr = a + b + c;
            if prev < curr {
                bumps_3 += 1;
            }
            curr
        });

    println!("{}\n{}", bumps_1, bumps_3);
}
