use std::io::BufRead;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tile {
    Empty,
    East,
    South,
}

fn main() {
    let stdin = std::io::stdin();
    let mut grid = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'.' => Tile::Empty,
                    b'>' => Tile::East,
                    b'v' => Tile::South,
                    c => panic!("Unexpect byte: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    for i in 1.. {
        let mut moved = false;
        let mut grid_next = vec![vec![Tile::Empty; width]; height];
        // Move east
        {
            for r in 0..height {
                for c in 0..width {
                    let c_next = (c + 1) % width;
                    match grid[r][c] {
                        Tile::Empty => {}
                        Tile::East if Tile::Empty == grid[r][c_next] => {
                            grid_next[r][c_next] = Tile::East;
                            moved = true;
                        }
                        x => grid_next[r][c] = x,
                    }
                }
            }
        }
        grid = grid_next;

        let mut grid_next = vec![vec![Tile::Empty; width]; height];
        // Move south
        {
            for r in 0..height {
                let r_next = (r + 1) % height;
                for c in 0..width {
                    match grid[r][c] {
                        Tile::Empty => {}
                        Tile::South if Tile::Empty == grid[r_next][c] => {
                            grid_next[r_next][c] = Tile::South;
                            moved = true;
                        }
                        x => grid_next[r][c] = x,
                    }
                }
            }
        }
        grid = grid_next;

        if !moved {
            println!("{}", i);
            break;
        }
    }
}
