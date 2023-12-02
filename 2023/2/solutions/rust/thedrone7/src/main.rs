use std::io::{self, stdout, BufRead, Write};

const LIMITS: (u64, u64, u64) = (12, 13, 14);

fn main() {
    let contents = read_input();

    let mut total = 0;
    let mut total_power = 0;

    for line in contents.lines() {
        let parts = line.split(':').collect::<Vec<&str>>();
        let game_id = parts[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .unwrap();

        let turns = parts[1].split(';');

        let (red_limit, green_limit, blue_limit) = LIMITS;
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

        for turn in turns {
            let (mut red_count, mut green_count, mut blue_count) = (0, 0, 0);

            let cubes = turn.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
            for cube in cubes {
                let colors = cube.split(' ').collect::<Vec<&str>>();
                let color = colors[1];
                let count = colors[0].parse::<u64>().unwrap();

                match color {
                    "red" => red_count += count,
                    "green" => green_count += count,
                    "blue" => blue_count += count,
                    _ => {}
                }
            }

            if red_count > min_red {
                min_red = red_count;
            }

            if green_count > min_green {
                min_green = green_count;
            }

            if blue_count > min_blue {
                min_blue = blue_count;
            }
        }

        if min_red <= red_limit && min_green <= green_limit && min_blue <= blue_limit {
            total += game_id;
        }

        let game_power = min_red * min_green * min_blue;
        total_power += game_power;
    }

    stdout()
        .write_all(format!("{}\n{}\n", total, total_power).as_bytes())
        .unwrap();
}

fn read_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        input.push_str(&line);
        input.push_str("\n")
    }

    input
}
