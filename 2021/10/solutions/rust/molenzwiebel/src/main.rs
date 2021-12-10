use std::io::BufRead;

fn count_line(line: &str) -> (usize, usize) {
    // 256 long so that llvm can optimize out the bounds checks
    // should be enough for even the longest line
    let mut stack = [0u8; 256];
    let mut stack_ptr = 0u8;

    for chr in line.bytes() {
        if chr == b'[' || chr == b'<' || chr == b'(' || chr == b'{' {
            stack_ptr += 1;
            stack[stack_ptr as usize] = match chr {
                b'(' => chr + 1,
                _ => chr + 2,
            };
        } else {
            let expected = stack[stack_ptr as usize];
            if chr != expected {
                return (
                    match chr {
                        b')' => 3,
                        b']' => 57,
                        b'}' => 1197,
                        b'>' => 25137,
                        _ => unreachable!(),
                    },
                    0,
                );
            }

            stack_ptr -= 1;
        }
    }

    if stack_ptr != 0 {
        return (
            0,
            stack[1..=(stack_ptr as usize)]
                .iter()
                .rev()
                .map(|&x| match x {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, x| (acc * 5) + x),
        );
    }

    (0, 0)
}

fn main() {
    let entries = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| count_line(&x))
        .collect::<Vec<_>>();

    let first = entries.iter().map(|&(x, _)| x).sum::<usize>();
    let mut second = entries
        .iter()
        .filter(|x| x.1 != 0)
        .map(|x| x.1)
        .collect::<Vec<_>>();
    let middle = second.len() / 2;

    let (_, second, _) = second.select_nth_unstable(middle);

    println!("{}\n{}", first, second);
}
