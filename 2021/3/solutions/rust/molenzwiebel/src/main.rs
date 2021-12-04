use std::{cmp::Ordering, io::BufRead};

fn collect_bit_criteria(vec: &Vec<String>, condition: Ordering, on_eq_pick: bool) -> isize {
    let mut start = 0usize; // inclusive
    let mut end = vec.len(); // exclusive

    let mut bit_idx = 0;

    while start != end - 1 {
        // First, count the number of ones.
        let num_zeros = vec[start..end]
            .iter()
            .take_while(|&x| x.as_bytes()[bit_idx] == b'0')
            .count();

        let num_ones = (start..end).len() - num_zeros;

        let ord = num_ones.cmp(&num_zeros);

        if ord == condition || (ord == Ordering::Equal && on_eq_pick) {
            start += num_zeros;
        } else {
            end -= num_ones;
        }

        bit_idx += 1;
    }

    isize::from_str_radix(&vec[start], 2).unwrap()
}

fn main() {
    let mut input_lines = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    input_lines.sort();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..input_lines[0].len() {
        let count = input_lines
            .iter()
            .filter(|&x| x.as_bytes()[i] == b'0')
            .count();
        gamma_rate <<= 1;
        epsilon_rate <<= 1;

        if count * 2 > input_lines.len() {
            gamma_rate |= 1;
        } else {
            epsilon_rate |= 1;
        }
    }

    println!("{}", gamma_rate * epsilon_rate);
    println!(
        "{}",
        collect_bit_criteria(&input_lines, Ordering::Greater, true)
            * collect_bit_criteria(&input_lines, Ordering::Less, false)
    );
}
