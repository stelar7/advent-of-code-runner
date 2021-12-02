use std::io::BufRead;

enum Operation {
    Up,
    Down,
    Forward,
}

impl From<u8> for Operation {
    fn from(x: u8) -> Self {
        match x {
            b'u' => Operation::Up,
            b'd' => Operation::Down,
            b'f' => Operation::Forward,
            _ => panic!("Invalid operation"),
        }
    }
}

fn main() {
    let mut depth = 0;
    let mut forward = 0;

    let mut aim = 0;
    let mut aimed_depth = 0;

    std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.as_bytes();
            (line[0].into(), line[line.len() - 1] - b'0')
        })
        .for_each(|(op, val)| match op {
            Operation::Up => {
                depth -= val as i32;
                aim -= val as i32;
            }
            Operation::Down => {
                depth += val as i32;
                aim += val as i32;
            }
            Operation::Forward => {
                forward += val as i32;
                aimed_depth += aim * val as i32;
            }
        });

    println!("{}\n{}", depth * forward, aimed_depth * forward);
}
