use std::io::BufRead;

#[derive(Debug)]
pub enum Error {
    Corrupt(usize),
    Incomplete(usize),
}
impl Error {
    #[inline]
    fn incomplete_cost(opening_char: char) -> usize {
        match opening_char {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            c => panic!("Invalid char: {}", c),
        }
    }

    pub fn new_corrupt(closing_char: char) -> Self {
        match closing_char {
            ')' => Error::Corrupt(3),
            ']' => Error::Corrupt(57),
            '}' => Error::Corrupt(1197),
            '>' => Error::Corrupt(25137),
            c => panic!("Invalid char: {}", c),
        }
    }
    pub fn new_incomplete(opening_char: char) -> Self {
        Self::Incomplete(Self::incomplete_cost(opening_char))
    }
    pub fn chain_incomplete(self, opening_char: char) -> Self {
        match self {
            Self::Corrupt(score) => Self::Corrupt(score),
            Self::Incomplete(score) => {
                Self::Incomplete(5 * score + Self::incomplete_cost(opening_char))
            }
        }
    }
}

pub fn check(tokens: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Result<(), Error> {
    while let Some(opening_char @ ('(' | '[' | '{' | '<')) = tokens.peek().cloned() {
        tokens.next();
        check(tokens).map_err(|err| err.chain_incomplete(opening_char))?;
        match (opening_char, tokens.next()) {
            ('(', Some(')')) => Ok(()),
            ('[', Some(']')) => Ok(()),
            ('{', Some('}')) => Ok(()),
            ('<', Some('>')) => Ok(()),
            (_, Some(c)) => Err(Error::new_corrupt(c)),
            (_, None) => Err(Error::new_incomplete(opening_char)),
        }?;
    }
    Ok(())
}

fn main() {
    let mut score_corrupt = 0;

    let mut score_incomplete_all = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| check(&mut line.chars().peekable()))
        .filter_map(|result| result.err())
        .filter_map(|err| match err {
            Error::Corrupt(score) => {
                score_corrupt += score;
                None
            }
            Error::Incomplete(score) => Some(score),
        })
        .collect::<Vec<_>>();
    let mid = (score_incomplete_all.len() - 1) / 2;
    let (_, &mut score_incomplete, _) = score_incomplete_all.select_nth_unstable(mid);

    println!("{}\n{}", score_corrupt, score_incomplete);
}
