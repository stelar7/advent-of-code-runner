use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
pub enum Move {
    Forward(u8),
    Down(u8),
    Up(u8),
}
impl FromStr for Move {
    type Err = Option<std::num::ParseIntError>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.split_once(' ') {
            Some(("forward", num)) => num.parse().map(Self::Forward).map_err(Some),
            Some(("down", num)) => num.parse().map(Self::Down).map_err(Some),
            Some(("up", num)) => num.parse().map(Self::Up).map_err(Some),
            _ => Err(None),
        }
    }
}

#[derive(Debug, Default)]
pub struct Position {
    pub position: usize,
    pub depth_basic: usize,
    pub depth_aimed: usize,
    pub aim: usize,
}

fn main() {
    let pos = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Failed to read line as UTF-8.")
                .parse::<Move>()
                .expect("Failed to parse move.")
        })
        .fold(Position::default(), |mut pos, curr_move| {
            match curr_move {
                Move::Forward(n) => {
                    pos.position += n as usize;
                    pos.depth_aimed += (n as usize) * pos.aim;
                },
                Move::Down(n) => {
                    pos.depth_basic += n as usize;
                    pos.aim += n as usize;
                },
                Move::Up(n) => {
                    pos.depth_basic -= n as usize;
                    pos.aim -= n as usize;
                },
            }
            pos
        });

    println!("{}\n{}", pos.position * pos.depth_basic, pos.position * pos.depth_aimed);
}
