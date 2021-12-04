use itertools::Itertools;
use std::io::BufRead;

struct BingoBoard {
    marks: i32,
    locations: [i32; 100],
    won: bool,
}

impl BingoBoard {
    fn new(board: [i32; 100]) -> Self {
        Self {
            locations: board,
            marks: 0,
            won: false,
        }
    }

    fn try_win(&mut self, value: usize) -> Option<usize> {
        let idx = self.locations[value];
        if idx == -1 {
            return None;
        }

        self.marks |= 1 << idx;

        if self.won || !self.is_complete() {
            return None;
        }

        let score = self
            .locations
            .iter()
            .enumerate()
            .filter(|(_, &x)| x != -1 && (self.marks & (1 << x)) == 0)
            .map(|x| x.0 as i32)
            .sum::<i32>();
        self.won = true;

        Some(score as usize)
    }

    fn is_complete(&self) -> bool {
        // horizontals
        (self.marks & 0b1111100000000000000000000) == 0b1111100000000000000000000 ||
        (self.marks & 0b0000011111000000000000000) == 0b0000011111000000000000000 ||
        (self.marks & 0b0000000000111110000000000) == 0b0000000000111110000000000 ||
        (self.marks & 0b0000000000000001111100000) == 0b0000000000000001111100000 ||
        (self.marks & 0b0000000000000000000011111) == 0b0000000000000000000011111 ||

        // verticals
        (self.marks & 0b1000010000100001000010000) == 0b1000010000100001000010000 ||
        (self.marks & 0b0100001000010000100001000) == 0b0100001000010000100001000 ||
        (self.marks & 0b0010000100001000010000100) == 0b0010000100001000010000100 ||
        (self.marks & 0b0001000010000100001000010) == 0b0001000010000100001000010 ||
        (self.marks & 0b0000100001000010000100001) == 0b0000100001000010000100001
    }
}

struct BingoSession {
    boards: Vec<BingoBoard>,
}

impl BingoSession {
    fn new(boards: Vec<BingoBoard>) -> Self {
        Self { boards }
    }

    fn mark(&mut self, number: usize) -> Option<usize> {
        let boards = &mut self.boards;

        let boards_won = boards
            .iter_mut()
            .flat_map(|x| x.try_win(number))
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
        .map(|x| {
            let mut locations = [-1; 100];

            for (idx, &val) in x.iter().enumerate() {
                locations[val] = idx as i32;
            }

            BingoBoard::new(locations)
        })
        .collect();

    let mut session = BingoSession::new(boards);

    let winners = input_nums
        .into_iter()
        .flat_map(|x| session.mark(x))
        .collect::<Vec<_>>();

    println!("{}\n{}", winners.first().unwrap(), winners.last().unwrap());
}
