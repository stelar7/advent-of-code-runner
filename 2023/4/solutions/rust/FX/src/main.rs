use std::{
    fmt::{Display, Write},
    io::stdin,
};

fn main() {
    let mut tickets = stdin().lines().map(Result::unwrap).peekable();

    let line1 = tickets.peek().unwrap();
    let start = line1.find(':').unwrap() + 2;
    let middle = line1.find('|').unwrap();
    let line_length = line1.len();

    let mut ticket_count = [1u32; 300];

    let (sum1, sum2) = tickets
        .into_iter()
        .enumerate()
        .map(|(i, ticket)| {
            // Card   1: 81  1 43 40 49 51 38 65 36  4 | 21 15  1 43 60  9 83 81 35 49 40 38 82 65 20  4 58 94 16 89 84 10 77 48 76\n
            //           ^                            ^  ^                                                                          ^
            //           | 10                      40 |  | 42                                                                   116 |

            let winning_numbers_str = &ticket[start..(middle - 1)];
            let my_numbers_str = &ticket[(middle + 2)..line_length];

            // println!("{}\n{}", winning_numbers_str, my_numbers_str);

            let mut bitset = Bitset::new();

            for i in (0..winning_numbers_str.len()).step_by(3) {
                let number = parse_int(&winning_numbers_str[i..i + 2]);
                bitset.set(number);
            }

            let mut matching_numbers = 0;
            for i in (0..my_numbers_str.len()).step_by(3) {
                let number = parse_int(&my_numbers_str[i..i + 2]);
                matching_numbers += u8::from(bitset.is_set(number));
            }

            for j in (i + 1)..=(i + usize::from(matching_numbers)) {
                ticket_count[j] += ticket_count[i];
            }

            (
                points_from_matching_numbers(matching_numbers),
                ticket_count[i],
            )
        })
        .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
        .unwrap();

    println!("{sum1}\n{sum2}");
}

struct Bitset(u128);

impl Bitset {
    #[inline(always)]
    fn new() -> Self {
        Self(0)
    }

    #[inline(always)]
    fn set(&mut self, bit: u8) {
        self.0 |= 1 << bit;
    }

    #[inline(always)]
    fn is_set(&self, bit: u8) -> bool {
        self.0 & 1 << bit != 0
    }
}

impl Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..128 {
            f.write_char(char::from(b'0' + u8::from(self.is_set(i))))?;
        }
        Ok(())
    }
}

// 'str' has to contain atleast 2 bytes
#[inline(always)]
fn parse_int(str: &str) -> u8 {
    let bytes = str.as_bytes();
    // bytes[0] might be a space => saturation_sub prevents underflow and returns 0 instead
    (bytes[0].saturating_sub(b'0')) * 10 + (bytes[1] - b'0')
}

#[inline(always)]
fn points_from_matching_numbers(matching_numbers: u8) -> u32 {
    u32::from(matching_numbers != 0) << (matching_numbers.saturating_sub(1))
}
