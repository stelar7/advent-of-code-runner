use std::cmp::min;
use std::io::BufRead;

const SIZE: usize = 10;

fn main() {
    let mut grid = [[0_i8; SIZE]; SIZE];

    let mut big_flash = None;

    for (r, line) in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .enumerate()
    {
        for (c, n) in line.bytes().map(|c| c - b'0').enumerate() {
            grid[r][c] = n.try_into().unwrap();
        }
    }

    let mut flashes_100 = 0;

    for step in 1.. {
        if 100 < step && big_flash.is_some() {
            break;
        }

        #[allow(clippy::needless_range_loop)]
        for r in 0..SIZE {
            for c in 0..SIZE {
                grid[r][c] += 1;
            }
        }

        while {
            let mut changed = false;
            for r in 0..SIZE {
                for c in 0..SIZE {
                    if 10 <= grid[r][c] {
                        #[allow(clippy::needless_range_loop)]
                        for r2 in r.saturating_sub(1)..min(SIZE, r + 2) {
                            for c2 in c.saturating_sub(1)..min(SIZE, c + 2) {
                                grid[r2][c2] += 1;
                            }
                        }
                        grid[r][c] = i8::MIN;
                        changed = true;
                    }
                }
            }
            changed
        } {}

        let mut flashes_step = 0;
        #[allow(clippy::needless_range_loop)]
        for r in 0..SIZE {
            for c in 0..SIZE {
                if grid[r][c] < 0 {
                    flashes_step += 1;
                    grid[r][c] = 0;
                }
            }
        }

        if SIZE * SIZE == flashes_step {
            big_flash.get_or_insert(step);
        }
        if step <= 100 {
            flashes_100 += flashes_step;
        }
    }

    println!("{}\n{}", flashes_100, big_flash.unwrap());
}
