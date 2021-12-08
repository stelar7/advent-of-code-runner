use std::collections::HashMap;
use std::io::BufRead;

const SEGS: usize = 7;

fn parse_segs(letters: &str) -> [bool; SEGS] {
    let mut segs = [false; SEGS];
    for letter in letters.bytes() {
        let i = letter - b'a';
        segs[i as usize] = true;
    }
    segs
}

fn main() {
    let (count, number) = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let mut segs_2_digit: HashMap<[bool; SEGS], u8> = Default::default();

            let (ten_digits, four_digits) =
                line.split_once(" | ").expect("Bad syntax: missing bar.");

            let segs_list = ten_digits
                .split(' ')
                .map(|letters| {
                    let segs = parse_segs(letters);
                    match letters.len() {
                        2 => segs_2_digit.insert(segs, 1),
                        4 => segs_2_digit.insert(segs, 4),
                        3 => segs_2_digit.insert(segs, 7),
                        7 => segs_2_digit.insert(segs, 8),
                        _ => None,
                    };
                    segs
                })
                .collect::<Vec<_>>();

            let segs_bitcounts = segs_list
                .iter()
                .map(|bool_arr| bool_arr.map(|b| b as u8))
                .fold([0; SEGS], |mut a, b| {
                    for i in 0..SEGS {
                        a[i] += b[i];
                    }
                    a
                });

            let mut seg_occ_4 = 0;
            let mut seg_occ_6 = 0;
            let mut seg_occ_8 = 0;
            #[allow(clippy::needless_range_loop)]
            for i in 0..SEGS {
                match segs_bitcounts[i] {
                    4 => seg_occ_4 = i,
                    6 => seg_occ_6 = i,
                    8 => seg_occ_8 = i,
                    _ => (),
                };
            }

            println!("{:?}", segs_bitcounts);

            for segs in segs_list.iter() {
                // 0 1 1
                if !segs[seg_occ_6] && segs[seg_occ_8] && segs[seg_occ_4] {
                    segs_2_digit.insert(*segs, 2);
                }
                // 1 0 1
                else if segs[seg_occ_6] && !segs[seg_occ_8] && segs[seg_occ_4] {
                    segs_2_digit.insert(*segs, 6);
                }
                // 1 0 0
                else if segs[seg_occ_6] && !segs[seg_occ_8] && !segs[seg_occ_4] {
                    segs_2_digit.insert(*segs, 5);
                }
                // Collisions
                else if !segs_2_digit.contains_key(segs) {
                    // 1 1 1
                    if segs[seg_occ_6] && segs[seg_occ_8] && segs[seg_occ_4] {
                        segs_2_digit.insert(*segs, 0);
                    }
                    // 0 1 0
                    else if !segs[seg_occ_6] && segs[seg_occ_8] && !segs[seg_occ_4] {
                        segs_2_digit.insert(*segs, 3);
                    }
                    // 1 1 0
                    else if segs[seg_occ_6] && segs[seg_occ_8] && !segs[seg_occ_4] {
                        segs_2_digit.insert(*segs, 9);
                    }
                }
            }

            let mut digits = segs_2_digit.iter().map(|(a, b)| (*b, *a)).collect::<Vec<_>>();
            digits.sort_unstable();
            println!("{:?}", digits);

            let output_digits = four_digits
                .split(' ')
                .map(parse_segs)
                .map(|segs| *segs_2_digit.get(&segs).unwrap())
                .collect::<Vec<_>>();
            output_digits
        })
        .map(|output_digits| {
            let count = output_digits
                .iter()
                .filter(|&&digit| 1 == digit || 4 == digit || 7 == digit || 8 == digit)
                .count();
            let number = output_digits
                .iter()
                .fold(0_usize, |a, b| 10 * a + (*b as usize));
            println!("{}", number);
            (count, number)
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}\n{}", count, number);
}
