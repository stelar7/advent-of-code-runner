use std::io::stdin;

fn main() {
    let (sum1, sum2): (u32, u32) = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            (
                (find_first1(&line) * 10 + find_last1(&line)).into(),
                (find_first2(&line) * 10 + find_last2(&line)).into(),
            )
        })
        .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
        .unwrap();

    println!("{sum1}\n{sum2}");
}

const NUMBERS: [&[u8]; 10] = [
    b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

#[inline(always)]
fn find_first1(line: &str) -> u8 {
    for byte in line.as_bytes() {
        if byte.is_ascii_digit() {
            return byte - b'0';
        }
    }

    0
}

#[inline(always)]
fn find_last1(line: &str) -> u8 {
    for byte in line.as_bytes().iter().rev() {
        if byte.is_ascii_digit() {
            return byte - b'0';
        }
    }

    0
}

#[inline(always)]
fn find_first2(line: &str) -> u8 {
    let bytes = line.as_bytes();

    for i in 0..bytes.len() {
        let byte = bytes[i];
        if byte.is_ascii_digit() {
            return byte - b'0';
        }

        // skip if the remaining bytes aren't long enough to fit a word anymore
        if bytes.len() - i < 3 {
            continue;
        }

        for (j, number) in NUMBERS.iter().enumerate() {
            if bytes[i..].starts_with(number) {
                return j as u8;
            }
        }
    }

    0
}

#[inline(always)]
fn find_last2(line: &str) -> u8 {
    let bytes = line.as_bytes();

    for i in (0..bytes.len()).rev() {
        let byte = bytes[i];
        if byte.is_ascii_digit() {
            return byte - b'0';
        }

        // skip if the remaining bytes aren't long enough to fit a word anymore
        if i < 3 {
            continue;
        }

        for (j, number) in NUMBERS.iter().enumerate().rev() {
            if bytes[..=i].ends_with(number) {
                return j as u8;
            }
        }
    }

    0
}
