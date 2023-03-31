use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

const fn score(byte: u8) -> u32 {
    (if byte < b'a' {
        byte - b'A' + 27
    } else {
        byte - b'a' + 1
    }) as u32
}

fn main() {
    let (p1, p2) = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .tuples()
        .map(|(l1, l2, l3)| {
            let p1: u32 = [&*l1, &*l2, &*l3]
                .into_iter()
                .map(|line| {
                    let mid = line.len() / 2;
                    let a = &line[..mid];
                    let b = &line[mid..];
                    let a_bytes: HashSet<_> = a.as_bytes().iter().copied().collect();
                    let shared_byte = b
                        .as_bytes()
                        .iter()
                        .copied()
                        .find(|byte| a_bytes.contains(byte))
                        .unwrap();
                    score(shared_byte)
                })
                .sum();

            let p2 = {
                let l1_bytes: HashSet<_> = l1.as_bytes().iter().copied().collect();
                let l2_bytes: HashSet<_> = l2.as_bytes().iter().copied().collect();
                let shared_byte = l3
                    .as_bytes()
                    .iter()
                    .copied()
                    .find(|byte| l1_bytes.contains(byte) && l2_bytes.contains(byte))
                    .unwrap();
                score(shared_byte)
            };

            (p1, p2)
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));
    println!("{}\n{}", p1, p2);
}
