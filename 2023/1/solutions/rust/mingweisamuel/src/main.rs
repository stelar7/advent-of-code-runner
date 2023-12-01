use regex::Regex;
use std::io::BufRead;

fn main() {
    let regex = Regex::new(r"(?:one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let (p1, p2) = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let p1 = {
                let mut digits = line
                    .bytes()
                    .filter(|&b| char::is_ascii_digit(&b.into()))
                    .map(|b| b - b'0');
                let a = digits.next().unwrap_or_default();
                let b = digits.next_back().unwrap_or(a);
                10 * a + b
            };
            let p2 = {
                let mut i = 0;
                let matches = std::iter::from_fn(|| {
                    let m = regex.find_at(&line, i)?;
                    i = m.start() + 1;
                    Some(m)
                });
                let mut nums = matches.map(|m| match m.as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    d => {
                        debug_assert_eq!(1, d.len());
                        d.bytes().next().unwrap() - b'0'
                    }
                });
                let a = nums.next().unwrap();
                let b = nums.last().unwrap_or(a);
                10 * a + b
            };
            (u32::from(p1), u32::from(p2))
        })
        .fold((0, 0), |(sa, sb), (a, b)| (sa + a, sb + b));

    println!("{}\n{}", p1, p2);
}
