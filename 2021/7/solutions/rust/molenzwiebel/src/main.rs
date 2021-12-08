use itertools::Itertools;
use std::io::Read;
use unroll::unroll_for_loops;

fn run(items: &mut [u16]) -> (i64, i64) {
    let (lt, median, gt) = items.select_nth_unstable(items.len() / 2);
    let mut min = u16::MAX;
    let mut max = *median;

    let mut linear = 0;

    for i in lt {
        linear += (*median - *i) as i64;
        min = min.min(*i);
    }

    for i in gt {
        linear += (*i - *median) as i64;
        max = max.max(*i);
    }

    let triangular = (min..=max)
        .map(|i| {
            items
                .iter()
                .map(|&pos| {
                    let dist = (pos as i64 - i as i64).abs();

                    (dist * (dist + 1)) / 2
                })
                .sum::<i64>()
        })
        .min()
        .unwrap();

    (linear as i64, triangular)
}

// parse a 4 digit number fast by assuming it is valid
#[unroll_for_loops]
fn parse_fast(input: &mut impl Iterator<Item = u8>) -> Option<u16> {
    let mut num = 0;

    if let Some(b) = input.next() {
        if b < b'0' || b > b'9' {
            return None;
        }

        num += (b - b'0') as u16;
    } else {
        return None;
    }

    for _ in 0..=2 {
        if let Some(b) = input.next() {
            if b < b'0' || b > b'9' {
                return Some(num);
            }

            num *= 10;
            num += (b - b'0') as u16;
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
        .collect::<Vec<_>>();

    let (a, b) = run(&mut items);
    println!("{}\n{}", a, b);
}
