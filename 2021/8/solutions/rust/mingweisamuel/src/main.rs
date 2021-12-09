use std::io::BufRead;

fn parse_segs(letters: &str) -> u8 {
    let mut segs = 0;
    for letter in letters.bytes() {
        let i = letter - b'a';
        segs |= 0b1 << i;
    }
    segs
}

fn main() {
    let (count, number) = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let mut segs_known = [0_u8; 10];

            let (ten_digits, four_digits) =
                line.split_once(" | ").expect("Bad syntax: missing bar.");

            let mut segs_unknown = ten_digits
                .split(' ')
                .map(parse_segs)
                .filter(|segs| {
                    match segs.count_ones() {
                        2 => segs_known[1] = *segs,
                        4 => segs_known[4] = *segs,
                        3 => segs_known[7] = *segs,
                        7 => segs_known[8] = *segs,
                        _ => return true,
                    };
                    false
                })
                .collect::<Vec<_>>();

            segs_unknown.retain(|&segs| {
                if segs_known[4] == segs & segs_known[4] {
                    segs_known[9] = segs;
                    false
                } else if segs_known[7] == segs & segs_known[7] {
                    match segs.count_ones() {
                        6 => segs_known[0] = segs,
                        5 => segs_known[3] = segs,
                        _ => unreachable!(),
                    }
                    false
                } else if 6 == segs.count_ones() {
                    segs_known[6] = segs;
                    false
                } else {
                    true
                }
            });
            for segs in segs_unknown {
                if 5 == (segs_known[6] & segs).count_ones() {
                    segs_known[5] = segs;
                } else {
                    segs_known[2] = segs;
                }
            }
            assert!(!segs_known.contains(&0));

            let output_digits = four_digits
                .split(' ')
                .map(parse_segs)
                .map(|segs| segs_known.iter().position(|&s| s == segs).unwrap())
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
            (count, number)
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}\n{}", count, number);
}
