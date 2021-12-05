use bitvec::prelude::*;
use itertools::Itertools;
use std::io::BufRead;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

type Line = ((usize, usize), (usize, usize));

#[inline(always)]
fn xy_to_i(x: usize, y: usize) -> usize {
    y * WIDTH + x
}

fn flag(flagged: &mut BitSlice, seen: &mut BitSlice, line: Line) -> usize {
    let mut count = 0;

    let ((x0, y0), (x1, y1)) = line;

    let mut x = x0;
    let mut y = y0;

    loop {
        let i = xy_to_i(x, y);
        let now_done = x == x1 && y == y1;

        if x != x1 {
            if x0 > x1 {
                x -= 1;
            } else {
                x += 1
            }
        }

        if y != y1 {
            if y0 > y1 {
                y -= 1;
            } else {
                y += 1
            }
        }

        // flag on first occurrence
        if !flagged[i] {
            flagged.set(i, true);
        } else if !seen[i] {
            // already flagged but not yet counted
            seen.set(i, true);
            count += 1;
        }

        if now_done {
            break;
        }
    }

    count
}

// parse a 3 digit number fast by assuming it is valid
fn parse_fast(input: &mut impl Iterator<Item = char>) -> usize {
    let mut num = 0;

    num += (input.next().unwrap() as u8 - b'0') as usize;
    if let Some(b) = input.next() {
        num *= 10;
        num += (b as u8 - b'0') as usize;
    }
    if let Some(b) = input.next() {
        num *= 10;
        num += (b as u8 - b'0') as usize;
    }

    num
}

fn line_to_tuples(line: &str) -> Line {
    let mut iter = line.chars();

    let x1 = parse_fast(&mut iter.take_while_ref(|&x| x != ','));

    iter.next(); // ,

    let y1 = parse_fast(&mut iter.take_while_ref(|&x| x != ' '));

    iter.next(); // ' '
    iter.next(); // -
    iter.next(); // >
    iter.next(); // ' '

    let x2 = parse_fast(&mut iter.take_while_ref(|&x| x != ','));

    iter.next(); // ,

    let y2 = parse_fast(&mut iter);

    ((x1, y1), (x2, y2))
}

fn run(input: impl Iterator<Item = Line>, allow_diagonal: bool) -> usize {
    let mut flagged = bitarr![0; WIDTH * HEIGHT];
    let mut marked = bitarr![0; WIDTH * HEIGHT];

    input
        .filter(|x| allow_diagonal || x.0 .0 == x.1 .0 || x.0 .1 == x.1 .1)
        .map(|x| flag(&mut flagged, &mut marked, x))
        .sum()
}

fn main() {
    let input_lines = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|x| line_to_tuples(&x))
        .collect::<Vec<_>>();

    println!(
        "{}\n{}",
        run(input_lines.clone().into_iter(), false),
        run(input_lines.into_iter(), true)
    );
}
