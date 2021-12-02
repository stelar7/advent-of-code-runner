use std::io::BufRead;

#[derive(Debug, Default)]
pub struct Position {
    pub position: usize,
    pub depth_basic: usize,
    pub depth_aimed: usize,
    pub aim: usize,
}

#[derive(Debug)]
pub enum Move {
    Forward(u8),
    Down(u8),
    Up(u8),
}
impl std::str::FromStr for Move {
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
impl std::ops::AddAssign<Move> for Position {
    fn add_assign(&mut self, rhs: Move) {
        match rhs {
            Move::Forward(n) => {
                self.position += n as usize;
                self.depth_aimed += (n as usize) * self.aim;
            }
            Move::Down(n) => {
                self.depth_basic += n as usize;
                self.aim += n as usize;
            }
            Move::Up(n) => {
                self.depth_basic -= n as usize;
                self.aim -= n as usize;
            }
        }
    }
}
impl std::ops::Add<Move> for Position {
    type Output = Self;
    fn add(mut self, rhs: Move) -> Self::Output {
        self += rhs;
        self
    }
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
        .fold(Position::default(), |pos, curr_move| pos + curr_move);

    println!(
        "{}\n{}",
        pos.position * pos.depth_basic,
        pos.position * pos.depth_aimed
    );
}
