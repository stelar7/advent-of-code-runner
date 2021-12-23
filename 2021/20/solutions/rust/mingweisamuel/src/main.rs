use std::collections::HashMap;
use std::io::BufRead;

pub fn enhance(
    lookup: &[bool],
    grid: &HashMap<(i32, i32), bool>,
    background: bool,
) -> (HashMap<(i32, i32), bool>, bool) {
    let mut new_grid = HashMap::new();

    let &rn = grid.keys().map(|(r, _c)| r).min().unwrap();
    let &rx = grid.keys().map(|(r, _c)| r).max().unwrap();

    let &cn = grid.keys().map(|(_r, c)| c).min().unwrap();
    let &cx = grid.keys().map(|(_r, c)| c).max().unwrap();

    for r in (rn - 1)..=(rx + 1) {
        for c in (cn - 1)..=(cx + 1) {
            let mut val = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let b = grid.get(&(r + dr, c + dc)).copied().unwrap_or(background);
                    val <<= 1;
                    if b {
                        val += 1;
                    }
                }
            }
            new_grid.insert((r, c), lookup[val]);
        }
    }

    (new_grid, if background { lookup[511] } else { lookup[0] })
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected char: {}", c),
                })
                .collect::<Vec<_>>()
        });

    let lookup = lines.next().expect("Empty input");

    let mut grid = HashMap::new();
    for (r, row) in lines.skip(1).enumerate() {
        for (c, &b) in row.iter().enumerate() {
            grid.insert((r as i32, c as i32), b);
        }
    }

    let mut background = false;

    for i in 1..=50 {
        let grid_background = enhance(&*lookup, &grid, background);
        grid = grid_background.0;
        background = grid_background.1;

        if 2 == i {
            println!("{}", grid.values().filter(|&&b| b).count());
        }
    }

    println!("{}", grid.values().filter(|&&b| b).count());
}
