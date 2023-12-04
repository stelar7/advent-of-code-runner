use std::{cmp::max, io::BufRead};

fn main() {
    let (p1, p2) = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (game, subsets) = line.split_once(": ").expect("Expected ': '.");
            let game_id = game.strip_prefix("Game ").expect("Expected 'Game <id>'");
            let game_id: u32 = game_id.parse().expect("Expected '<id>' to be a number.");
            let [rx, gx, bx] = subsets
                .split("; ")
                .map(|subset| {
                    subset
                        .split(", ")
                        .map(|subset_color| {
                            let (count, color) = subset_color
                                .split_once(" ")
                                .expect("Expected '<count> <color>' format.");
                            let count: u32 = count.parse().unwrap_or_else(|e| {
                                panic!("Expected '<count>' number, got '{}': {}", count, e)
                            });
                            match color {
                                "red" => [count, 0, 0],
                                "green" => [0, count, 0],
                                "blue" => [0, 0, count],
                                _ => panic!("Expected 'red', 'green', or 'blue', got '{}'.", color),
                            }
                        })
                        .reduce(|[r1, g1, b1], [r2, g2, b2]| [r1 + r2, g1 + g2, b1 + b2])
                        .expect("Game was empty!")
                })
                .reduce(|[r1, g1, b1], [r2, g2, b2]| [max(r1, r2), max(g1, g2), max(b1, b2)])
                .unwrap();

            let p1 = if rx <= 12 && gx <= 13 && bx <= 14 {
                game_id
            } else {
                0
            };
            let p2 = rx * gx * bx;
            (p1, p2)
        })
        .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
        .unwrap();
    println!("{}\n{}", p1, p2);
}
