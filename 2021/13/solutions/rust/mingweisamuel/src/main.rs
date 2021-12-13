use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));

    let mut points: HashSet<(usize, usize)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .expect("Cannot parse line as coordinates, missing comma.");
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            (x, y)
        })
        .collect();

    let folds = lines.map(|line| {
        let (fold_along, n) = line
            .split_once('=')
            .expect("Cannot parse line as fold, missing equals.");
        let n = n.parse::<usize>().unwrap();
        let xy = fold_along
            .strip_prefix("fold along ")
            .expect("Cannot parse line as fold, missing \"fold along\".")
            .chars()
            .next()
            .unwrap();
        let is_x = match xy {
            'x' => true,
            'y' => false,
            c => panic!("Unknown axis {}.", c),
        };
        (is_x, n)
    });

    let mut visible = points.len();

    let mut bound_x = 0;
    let mut bound_y = 0;
    let mut visible_1 = None;
    for (is_x, n) in folds {
        points = points
            .iter()
            .cloned()
            .filter_map(|(x, y)| {
                let c = if is_x { x } else { y };
                if c < n {
                    Some((x, y))
                } else {
                    let dest = if is_x { (2 * n - x, y) } else { (x, 2 * n - y) };
                    if !points.contains(&dest) {
                        Some(dest)
                    } else {
                        visible -= 1;
                        None
                    }
                }
            })
            .collect();
        visible_1.get_or_insert(visible);

        if is_x {
            bound_x = n;
        }
        else {
            bound_y = n;
        }
    }

    println!("{}", visible_1.unwrap());

    for y in 0..bound_y {
        for x in 0..bound_x {
            print!(
                "{}",
                if points.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}
