use std::{collections::HashMap, io::stdin};

fn main() {
    let lines: Vec<_> = stdin().lines().map(Result::unwrap).collect();

    let mut sum1: u32 = 0;

    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut prev_line: Option<&[u8]> = None;
    let mut line_iter = lines.iter().enumerate().peekable();

    let mut gear_positions: Vec<(usize, usize)> = Vec::with_capacity(25);
    while let Some((i, line)) = line_iter.next() {
        let mut place_in_number: u32 = 1;
        let mut number: u32 = 0;
        let mut is_part_number = false;
        let next_line = line_iter.peek().map(|(_, line)| line.as_bytes());

        let mut prev_char = None;
        let mut char_iter = line.as_bytes().iter().enumerate().rev().peekable();
        while let Some((j, &char)) = char_iter.next() {
            if char.is_ascii_digit() {
                number += place_in_number * u32::from(digit_from_ascii_digit(char));
                place_in_number *= 10;

                if !is_part_number {
                    if let Some(prev_line) = prev_line {
                        if is_symbol(prev_line[j]) {
                            is_part_number = true;
                            gear_positions.push((i - 1, j));
                            continue;
                        }

                        if j > 0 && is_symbol(prev_line[j - 1]) {
                            is_part_number = true;
                            gear_positions.push((i - 1, j - 1));
                            continue;
                        }

                        if j < prev_line.len() - 1 && is_symbol(prev_line[j + 1]) {
                            is_part_number = true;
                            gear_positions.push((i - 1, j + 1));
                            continue;
                        }
                    }

                    if let Some(prev_char) = prev_char {
                        if is_symbol(prev_char) {
                            is_part_number = true;
                            gear_positions.push((i, j + 1));
                            continue;
                        }
                    }

                    if let Some((_, &next_char)) = char_iter.peek() {
                        if is_symbol(next_char) {
                            is_part_number = true;
                            gear_positions.push((i, j - 1));
                            continue;
                        }
                    }

                    if let Some(next_line) = next_line {
                        if is_symbol(next_line[j]) {
                            is_part_number = true;
                            gear_positions.push((i + 1, j));
                            continue;
                        }

                        if j > 0 && is_symbol(next_line[j - 1]) {
                            is_part_number = true;
                            gear_positions.push((i + 1, j - 1));
                            continue;
                        }

                        if j < next_line.len() - 1 && is_symbol(next_line[j + 1]) {
                            is_part_number = true;
                            gear_positions.push((i + 1, j + 1));
                            continue;
                        }
                    }
                }
            } else {
                if is_part_number {
                    sum1 += number;

                    for pos in &gear_positions {
                        if let Some(vec) = map.get_mut(pos) {
                            vec.push(number);
                        } else {
                            map.insert(*pos, vec![number]);
                        }
                    }
                }

                place_in_number = 1;
                number = 0;
                is_part_number = false;
                gear_positions.clear();
            }

            prev_char = Some(char);
        }

        if is_part_number {
            sum1 += number;

            for pos in &gear_positions {
                if let Some(vec) = map.get_mut(pos) {
                    vec.push(number);
                } else {
                    map.insert(*pos, vec![number]);
                }
            }
        }

        gear_positions.clear();

        prev_line = Some(line.as_bytes());
    }

    let mut sum2 = 0;
    for vec in map.values() {
        if vec.len() == 2 {
            sum2 += vec[0] * vec[1];
        }
    }

    println!("{sum1}\n{sum2}");
}

#[inline(always)]
fn digit_from_ascii_digit(ascii_digit: u8) -> u8 {
    ascii_digit - b'0'
}

#[inline(always)]
fn is_symbol(ascii_char: u8) -> bool {
    ascii_char != b'.' && ascii_char.is_ascii_punctuation()
}
