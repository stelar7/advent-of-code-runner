use std::{convert::TryInto, io::BufRead};

use itertools::Itertools;

struct BingoBoard {
    board: [usize; 25],
    won: bool,
}

impl BingoBoard {
    fn new(board: [usize; 25]) -> Self {
        Self { board, won: false }
    }

    fn try_win(&mut self, marked: &[bool; 100]) -> Option<usize> {
        if self.won || !self.is_complete(marked) {
            return None;
        }

        let score = self.board.iter().filter(|&&x| !marked[x]).sum();
        self.won = true;

        Some(score)
    }

    fn is_complete(&self, marked: &[bool; 100]) -> bool {
        let marked_nums = self
            .board
            .iter()
            .map(|&x| marked[x])
            .fold(0, |x, i| (x << 1) | (if i { 1 } else { 0 }));

        // horizontals
        (marked_nums & 0b1111100000000000000000000) == 0b1111100000000000000000000 ||
        (marked_nums & 0b0000011111000000000000000) == 0b0000011111000000000000000 ||
        (marked_nums & 0b0000000000111110000000000) == 0b0000000000111110000000000 ||
        (marked_nums & 0b0000000000000001111100000) == 0b0000000000000001111100000 ||
        (marked_nums & 0b0000000000000000000011111) == 0b0000000000000000000011111 ||

        // verticals
        (marked_nums & 0b1000010000100001000010000) == 0b1000010000100001000010000 ||
        (marked_nums & 0b0100001000010000100001000) == 0b0100001000010000100001000 ||
        (marked_nums & 0b0010000100001000010000100) == 0b0010000100001000010000100 ||
        (marked_nums & 0b0001000010000100001000010) == 0b0001000010000100001000010 ||
        (marked_nums & 0b0000100001000010000100001) == 0b0000100001000010000100001
    }
}

struct BingoSession {
    marked: [bool; 100], // 100 is the max number of unique numbers in a game
    boards: Vec<BingoBoard>,
}

impl BingoSession {
    fn new(boards: Vec<BingoBoard>) -> Self {
        Self {
            marked: [false; 100],
            boards,
        }
    }

    fn mark(&mut self, number: usize) -> Option<usize> {
        self.marked[number] = true;

        let boards = &mut self.boards;
        let marked = &self.marked;

        let boards_won = boards
            .iter_mut()
            .flat_map(|x| x.try_win(marked))
            .collect::<Vec<_>>();

        boards_won.first().map(|x| x * number)
    }
}

fn main() {
    let mut input_lines = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let input_nums = input_lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    input_lines.next(); // empty line after numbers

    let boards: Vec<_> = input_lines
        .batching(|x| {
            let board_lines = x.take_while(|y| !y.is_empty()).collect::<Vec<_>>();

            match board_lines.is_empty() {
                true => None,
                false => Some(board_lines),
            }
        })
        .map(|x| {
            x.into_iter()
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<usize>>()
        })
        .map(|x| BingoBoard::new(x.try_into().unwrap()))
        .collect();

    let mut session = BingoSession::new(boards);

    let winners = input_nums
        .into_iter()
        .flat_map(|x| session.mark(x))
        .collect::<Vec<_>>();

    println!("{}\n{}", winners.first().unwrap(), winners.last().unwrap());
}
