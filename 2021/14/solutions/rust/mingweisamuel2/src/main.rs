use itertools::Itertools;
use std::io::BufRead;

const LETTERS: usize = 10;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));

    let polymer = lines.next().unwrap();

    let rules_vec: Vec<((u8, u8), u8)> = lines
        .skip(1)
        .map(|line| {
            let (from, to) = line.split_once(" -> ").expect("Line missing arrow.");
            assert_eq!(2, from.len());
            assert_eq!(1, to.len());
            let from = from.as_bytes();
            let from_a = from[0] - b'A';
            let from_b = from[1] - b'A';
            let to = to.as_bytes()[0] - b'A';
            ((from_a, from_b), to)
        })
        .collect();

    let letters = rules_vec
        .iter()
        .flat_map(|&((a, b), _)| [a, b])
        .collect::<std::collections::BTreeSet<_>>();

    let num_letters = letters.len();
    assert!(num_letters <= LETTERS);

    let mut lookup = [0_u8; 26];
    for (i, &letter) in letters.iter().enumerate() {
        lookup[letter as usize] = i as u8;
    }

    let mut rules = [[0_u8; LETTERS]; LETTERS];
    for ((from_a, from_b), to) in rules_vec {
        let from_a = lookup[from_a as usize];
        let from_b = lookup[from_b as usize];
        let to = lookup[to as usize];
        rules[from_a as usize][from_b as usize] = to;
    }

    let prev = &mut [[[0_usize; LETTERS]; LETTERS]; LETTERS];
    let next = &mut [[[0_usize; LETTERS]; LETTERS]; LETTERS];

    let mut sol_10 = 0;
    let mut sol_40 = 0;

    for gen in 1..=40 {
        for a in 0..num_letters {
            for b in 0..num_letters {
                let new_char = rules[a][b];
                for c in 0..num_letters {
                    let n = prev[a as usize][new_char as usize][c];
                    let m = prev[new_char as usize][b as usize][c];
                    next[a as usize][b as usize][c] = n + m;
                }
                next[a as usize][b as usize][new_char as usize] += 1;
            }
        }

        if let 10 | 40 = gen {
            let mut count = [0; LETTERS];
            for letter in polymer.bytes() {
                let a = lookup[(letter - b'A') as usize];
                count[a as usize] += 1;
            }
            for (letter_a, letter_b) in polymer.bytes().tuple_windows() {
                let a = lookup[(letter_a - b'A') as usize];
                let b = lookup[(letter_b - b'A') as usize];

                #[allow(clippy::needless_range_loop)]
                for c in 0..num_letters {
                    count[c] += next[a as usize][b as usize][c];
                }
            }
            let count = &count[..num_letters];
            let sol = count.iter().max().unwrap() - count.iter().min().unwrap();
            if 10 == gen {
                sol_10 = sol;
            }
            else {
                sol_40 = sol;
            }
        }

        std::mem::swap(prev, next);
    }
    println!("{}\n{}", sol_10, sol_40);
}
