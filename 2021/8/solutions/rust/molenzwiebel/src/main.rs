use std::io::BufRead;
use unroll::unroll_for_loops;

type Entry = ([u8; 10], [u8; 4]);

fn evaluate_entry(entry: &Entry) -> usize {
    let (inputs, outputs) = entry;

    // unique patterns
    // ---
    let one = *inputs.iter().find(|&&x| x.count_ones() == 2).unwrap();
    let four = *inputs.iter().find(|&&x| x.count_ones() == 4).unwrap();
    let seven = *inputs.iter().find(|&&x| x.count_ones() == 3).unwrap();
    let eight = *inputs.iter().find(|&&x| x.count_ones() == 7).unwrap();

    let mut ret = 0;
    for &i in outputs {
        let num_bits = i.count_ones();

        // Time to check.
        let val = if num_bits == 6 && (i & one == one) && (i & four != four) {
            // zero has 6 segments active, contains one, but does not contain four
            0
        } else if i == one {
            1
        } else if num_bits == 5 && (i & four).count_ones() == 2 {
            // two has five segments active and contains two common bits with four
            2
        } else if num_bits == 5 && (i & one) == one {
            // three has five segments active and contains one
            3
        } else if i == four {
            4
        } else if num_bits == 5 {
            // five is the only number left with 5 segments active, as we've already
            // tied two and three
            5
        } else if num_bits == 6 && (i & one) != one {
            // six has six segments active, but does not contain one
            6
        } else if i == seven {
            7
        } else if i == eight {
            8
        } else {
            9
        };

        ret = ret * 10 + val;
    }

    ret
}

#[unroll_for_loops]
fn parse_entry(line: &str) -> Entry {
    let mut patterns = [0u8; 10];
    let mut output = [0u8; 4];

    let mut iter = line.bytes();

    for i in 0..10 {
        loop {
            let c = iter.next().unwrap();
            if c == b' ' {
                break;
            }

            patterns[i] |= 1 << (c - 'a' as u8);
        }
    }

    iter.next(); // '|'
    iter.next(); // ' '

    for i in 0..4 {
        loop {
            let c = match iter.next() {
                Some(x) => x,
                None => break,
            };

            if c == b' ' {
                break;
            }

            output[i] |= 1 << (c - 'a' as u8);
        }
    }

    (patterns, output)
}

fn main() {
    let entries = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse_entry(&x))
        .collect::<Vec<_>>();

    let simple_digits = entries
        .iter()
        .map(|&(_, x)| {
            x.iter()
                .filter(|x| {
                    let cnt = x.count_ones();
                    matches!(cnt, 2 | 3 | 4 | 7)
                })
                .count()
        })
        .sum::<usize>();

    let complex = entries.iter().map(evaluate_entry).sum::<usize>();

    println!("{}\n{}", simple_digits, complex);
}
