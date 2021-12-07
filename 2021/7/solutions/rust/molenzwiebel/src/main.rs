use itertools::Itertools;
use rdxsort::RdxSort;
use std::io::Read;
use unroll::unroll_for_loops;

#[inline(never)]
fn run(items: &Vec<usize>) -> (i64, i64) {
    let min = items[0] as i64;
    let max = items[items.len() - 1] as i64;
    let median = items[items.len() / 2] as i64;

    let linear = items.iter().map(|&x| (x as i64 - median).abs()).sum();
    let triangular = (min..=max)
        .map(|i| {
            items
                .iter()
                .map(|&pos| {
                    let dist = (pos as i64 - i).abs();

                    (dist * (dist + 1)) / 2
                })
                .sum::<i64>()
        })
        .min()
        .unwrap();

    (linear, triangular)
}

// parse a 4 digit number fast by assuming it is valid
#[unroll_for_loops]
fn parse_fast(input: &mut impl Iterator<Item = u8>) -> Option<usize> {
    let mut num = 0;

    if let Some(b) = input.next() {
        if b < b'0' || b > b'9' {
            return None;
        }

        num += (b - b'0') as usize;
    } else {
        return None;
    }

    for _ in 0..=2 {
        if let Some(b) = input.next() {
            if b < b'0' || b > b'9' {
                return Some(num);
            }

            num *= 10;
            num += (b - b'0') as usize;
        }
    }

    input.next(); // skip comma

    Some(num)
}

fn main() {
    let mut items = std::io::stdin()
        .lock()
        .bytes()
        .map(|b| b.unwrap())
        .batching(parse_fast)
        .collect::<Vec<usize>>();

    items.rdxsort();

    let (a, b) = run(&items);
    println!("{}\n{}", a, b);
}
