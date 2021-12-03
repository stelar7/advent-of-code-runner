use bitvec::vec::BitVec;
use std::io::BufRead;

fn as_usize(bv: &BitVec) -> usize {
    bv.iter().fold(0, |n, x| (n << 1) + (*x as usize))
}
fn common_bit(rows: &[&BitVec], i: usize) -> bool {
    let occ = rows.iter().fold(0, |occ, row| occ + (row[i] as usize));
    rows.len() <= 2 * occ
}
fn filter_bit(rows: Vec<&BitVec>, i: usize, bit: bool) -> Vec<&BitVec> {
    rows.into_iter().filter(|row| bit == row[i]).collect()
}

fn find_rating(mut rows: Vec<&BitVec>, common: bool) -> Option<&BitVec> {
    let width = rows.first()?.len();
    for i in 0..width {
        if rows.len() <= 1 {
            break;
        }
        let cb = common_bit(&*rows, i);
        rows = filter_bit(rows, i, common ^ cb);
    }
    if 1 < rows.len() {
        None
    } else {
        rows.pop()
    }
}

fn main() {
    let rows: Vec<BitVec> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Failed to read line as UTF-8.")
                .chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let count = rows.len();
    let width = rows.first().expect("Empty input.").len();

    let commons: BitVec = rows
        .iter()
        .fold(vec![0_usize; width], |mut occ, row| {
            for i in 0..width {
                occ[i] += row[i] as usize;
            }
            occ
        })
        .into_iter()
        .map(|occ| count <= 2 * occ)
        .collect();

    let gamma = as_usize(&commons);
    let epsilon = ((0b1 << width) - 1) & (!gamma);

    let generator = find_rating(rows.iter().collect(), true);
    let generator = as_usize(generator.expect("Failed to determine generator rating."));
    let scrubber = find_rating(rows.iter().collect(), false);
    let scrubber = as_usize(scrubber.expect("Failed to determine scrubber rating."));

    println!("{}\n{}", gamma * epsilon, generator * scrubber);
}
