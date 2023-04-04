use std::io::BufRead;

pub fn main() {
    let mut power: i32 = 0;
    let mut cycle: i32 = 1;
    let mut reg: i32 = 1;
    let mut grid = [false; 40 * 6];
    grid[0] = true;
    for line in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
    {
        if "noop" == line {
            cycle += 1;
        } else {
            let add: i32 = line.strip_prefix("addx ").unwrap().parse().unwrap();
            cycle += 1;
            // Start of next cycle
            if 20 == cycle % 40 {
                power += reg * cycle;
            }
            if reg.abs_diff((cycle - 1) % 40) <= 1 {
                grid[cycle as usize - 1] = true;
            }
            cycle += 1;
            reg += add;
        }
        // Start of next cycle
        if 20 == cycle % 40 {
            power += reg * cycle;
        }
        if reg.abs_diff((cycle - 1) % 40) <= 1 {
            grid[cycle as usize - 1] = true;
        }
    }

    println!("{}", power);
    for r in 0..6 {
        for c in 0..40 {
            print!("{}", if grid[r * 40 + c] { '#' } else { '.' });
        }
        println!();
    }
}
