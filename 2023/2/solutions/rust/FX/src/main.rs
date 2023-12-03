use std::io::stdin;

fn main() {
    let (sum1, sum2) = stdin()
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(i, line)| {
            // split up line
            let (_, line_after_colon) = line[6..].split_once(':').unwrap();
            let draws = line_after_colon.split(';').map(|draw| {
                draw.split(',')
                    .map(|cube| cube[1..].split_once(' ').unwrap())
            });

            let mut return_value = i + 1;

            // [r, g, b]
            let mut max_colors: [u32; 3] = [0, 0, 0];
            for draw in draws {
                // [r, g, b]
                let mut colors: [u32; 3] = [0, 0, 0];

                // parse (color, count) and sum up in colors[]
                for (count, color) in draw {
                    let idx = color_to_index(color);
                    let count = count.parse::<u32>().unwrap();

                    colors[idx] += count;
                    max_colors[idx] = u32::max(max_colors[idx], count);
                }

                // check if one of the sums is over the limit
                if colors[0] > 12 || colors[1] > 13 || colors[2] > 14 {
                    return_value = 0;
                }
            }

            let power = max_colors.into_iter().reduce(|a, b| a * b).unwrap();

            (return_value, power)
        })
        .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
        .unwrap();

    println!("{sum1}\n{sum2}");
}

#[inline(always)]
fn color_to_index(color: &str) -> usize {
    match color.as_bytes()[0] {
        b'r' => 0,
        b'g' => 1,
        b'b' => 2,
        _ => unreachable!(),
    }
}
