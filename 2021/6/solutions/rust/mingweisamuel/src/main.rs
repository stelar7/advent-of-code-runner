use std::io::BufRead;

const FISHTYPES: usize = 9;

pub const fn fish_counts<const N: usize>() -> [usize; N] {
    let mut fish = [[0; FISHTYPES]; 2];
    let mut hist = [0; N];

    fish[0][8] = 1;
    hist[0] = 1;

    let mut i = 1;
    while i < N {
        let prev = (i - 1) % 2;
        let curr = i % 2;
        fish[curr][0] = fish[prev][1];
        fish[curr][1] = fish[prev][2];
        fish[curr][2] = fish[prev][3];
        fish[curr][3] = fish[prev][4];
        fish[curr][4] = fish[prev][5];
        fish[curr][5] = fish[prev][6];
        fish[curr][6] = fish[prev][7] + fish[prev][0];
        fish[curr][7] = fish[prev][8];
        fish[curr][8] = fish[prev][0];

        hist[i] = fish[curr][0]
            + fish[curr][1]
            + fish[curr][2]
            + fish[curr][3]
            + fish[curr][4]
            + fish[curr][5]
            + fish[curr][6]
            + fish[curr][7]
            + fish[curr][8];

        i += 1;
    }

    hist
}

const DAYS_A: usize = 80;
const DAYS_B: usize = 256;

const N: usize = DAYS_B + FISHTYPES;
const FISH_COUNTS: [usize; N] = fish_counts();

pub const fn spawned(days: usize, fish: usize) -> usize {
    let i = days + 8 - fish;
    FISH_COUNTS[i]
}

fn main() {
    let line = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .next()
        .expect("Empty input");

    let (count_a, count_b) = line
        .split(',')
        .map(|num_str| num_str.parse::<usize>().expect("Failed to parse integer."))
        .map(|fish| (spawned(DAYS_A, fish), spawned(DAYS_B, fish)))
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}\n{}", count_a, count_b);
}
