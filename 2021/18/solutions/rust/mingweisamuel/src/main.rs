use std::io::BufRead;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
pub enum SnailfishNumber {
    Literal(u32),
    Pair(Box<(SnailfishNumber, SnailfishNumber)>),
}
impl SnailfishNumber {
    pub const MAX_DEPTH: u32 = 4;

    pub fn parse(chars: &mut impl Iterator<Item = char>) -> SnailfishNumber {
        let out;
        match chars.next() {
            Some('[') => {
                let lhs = Self::parse(chars);
                assert_eq!(Some(','), chars.next());
                let rhs = Self::parse(chars);
                assert_eq!(Some(']'), chars.next());
                out = SnailfishNumber::Pair(Box::new((lhs, rhs)));
            }
            Some(digit_char) => {
                let num = digit_char.to_digit(10).expect("Failed to parse literal") as u32;
                out = SnailfishNumber::Literal(num);
            }
            None => panic!("Unexpected EOS."),
        }
        out
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Self::Literal(num) => *num,
            Self::Pair(boxed) => {
                let (lhs, rhs) = &**boxed;
                3 * lhs.magnitude() + 2 * rhs.magnitude()
            }
        }
    }

    pub fn reduce(&mut self) {
        while self.reduce_nested(0).is_some() || self.reduce_tens() {}
    }

    fn reduce_nested(&mut self, depth: u32) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Self::Literal(_num) => None,
            Self::Pair(boxed) => {
                let (lhs, rhs) = &mut **boxed;
                if Self::MAX_DEPTH == depth {
                    if let (Self::Literal(ref a), Self::Literal(ref b)) = (lhs, rhs) {
                        let (a, b) = (*a, *b);
                        *self = Self::Literal(0);
                        Some((Some(a), Some(b)))
                    } else {
                        panic!("Depth exceeded 4");
                    }
                } else {
                    match lhs.reduce_nested(depth + 1) {
                        Some((opt_explode_left, Some(explode_right))) => {
                            rhs.add_left(explode_right);
                            return Some((opt_explode_left, None));
                        }
                        Some(other) => return Some(other),
                        None => (),
                    }
                    match rhs.reduce_nested(depth + 1) {
                        Some((Some(explode_left), opt_explode_right)) => {
                            lhs.add_right(explode_left);
                            return Some((None, opt_explode_right));
                        }
                        Some(other) => return Some(other),
                        None => (),
                    }
                    None
                }
            }
        }
    }

    /**
     * Adds VAL to the leftmost branch of the number.
     */
    fn add_left(&mut self, val: u32) {
        match self {
            Self::Literal(num) => *num += val,
            Self::Pair(boxed) => {
                let (lhs, _rhs) = &mut **boxed;
                lhs.add_left(val);
            }
        }
    }

    /**
     * Adds VAL to the rightmost branch of the number.
     */
    fn add_right(&mut self, val: u32) {
        match self {
            Self::Literal(num) => *num += val,
            Self::Pair(boxed) => {
                let (_lhs, rhs) = &mut **boxed;
                rhs.add_right(val);
            }
        }
    }

    fn reduce_tens(&mut self) -> bool {
        match self {
            Self::Literal(num) => {
                let num = *num;
                if 10 <= num {
                    let (q, r) = (num / 2, num % 2);
                    let a = q;
                    let b = q + r;
                    *self = Self::Pair(Box::new((Self::Literal(a), Self::Literal(b))));
                    true
                } else {
                    false
                }
            }
            Self::Pair(boxed) => {
                let (lhs, rhs) = &mut **boxed;
                lhs.reduce_tens() || rhs.reduce_tens()
            }
        }
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = SnailfishNumber::Pair(Box::new((self, rhs)));
        out.reduce();
        out
    }
}

fn main() {
    let stdin = std::io::stdin();
    let numbers: Vec<_> = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| SnailfishNumber::parse(&mut line.chars()))
        .collect();

    let sum = numbers
        .iter()
        .cloned()
        .reduce(|a, b| a + b)
        .expect("Empty input.");

    let mut max_mag = 0;
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers.iter().cloned().enumerate() {
            if i != j {
                max_mag = std::cmp::max(max_mag, (a.clone() + b).magnitude())
            }
        }
    }

    println!("{}\n{}", sum.magnitude(), max_mag);
}
