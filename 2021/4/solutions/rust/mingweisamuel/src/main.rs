use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
pub struct Bingo<const N: usize> {
    bingo: bool,
    sum: usize,
    row_counts: [usize; N],
    col_counts: [usize; N],
}
impl<const N: usize> Bingo<N> {
    pub fn new(sum: usize) -> Self {
        Self {
            bingo: false,
            sum,
            col_counts: [0; N],
            row_counts: [0; N],
        }
    }
    pub fn is_bingo(&self) -> bool {
        self.bingo
    }
    pub fn fill(&mut self, r: usize, c: usize, draw: usize) -> Option<usize> {
        self.row_counts[r] += 1;
        self.col_counts[c] += 1;
        self.sum -= draw;
        if N == self.row_counts[r] || N == self.col_counts[c] {
            self.bingo = true;
            Some(self.sum)
        } else {
            None
        }
    }
}

pub struct Index {
    pub board_idx: usize,
    pub r: usize,
    pub c: usize,
}

fn main() {
    const N: usize = 5;

    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));

    let draw_str = lines.by_ref().take(1).last().expect("Empty input.");
    let draws = draw_str.split(',').map(|num_str| {
        num_str
            .parse::<usize>()
            .expect("Failed to parse draw as integer.")
    });

    let mut index: HashMap<usize, Vec<Index>> = Default::default();

    let mut boards: Vec<Bingo<N>> = lines
        .skip(1)
        .chunks(N + 1)
        .into_iter()
        .enumerate()
        .map(|(board_idx, chunk)| {
            let mut sum = 0;
            for (r, line) in chunk.enumerate() {
                for (c, x) in line
                    .split_whitespace()
                    .map(|num_str| {
                        num_str
                            .parse::<usize>()
                            .expect("Failed to parse grid item as integer.")
                    })
                    .enumerate()
                {
                    sum += x;
                    // Build index:
                    index.entry(x).or_default().push(Index { board_idx, r, c });
                }
            }
            Bingo::new(sum)
        })
        .collect();

    let mut first = None;
    let mut last = None;
    for draw in draws {
        for idx in &index[&draw] {
            let board = &mut boards[idx.board_idx];
            if !board.is_bingo() {
                if let Some(sum) = board.fill(idx.r, idx.c, draw) {
                    let val = draw * sum;
                    first.get_or_insert(val);
                    last = Some(val);
                }
            }
        }
    }
    println!("{}\n{}", first.unwrap(), last.unwrap());
}
