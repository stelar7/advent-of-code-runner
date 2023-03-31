use std::cell::RefCell;
use std::io::BufRead;

use scan_fmt::scan_fmt;

fn main() {
    let mut lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));
    let grid_p1 = make_stacks(lines.by_ref().take_while(|line| !line.is_empty()));
    let grid_p2 = grid_p1.clone();

    for (count, src, dst) in lines
        .map(|line| scan_fmt!(&*line, "move {d} from {d} to {d}", usize, usize, usize).unwrap())
    {
        assert_ne!(src, dst);
        {
            let mut dst_stack = grid_p1[dst - 1].borrow_mut();
            let mut src_stack = grid_p1[src - 1].borrow_mut();
            let src_start = src_stack.len() - count;
            dst_stack.extend(src_stack.drain(src_start..).rev());
        }
        {
            let mut dst_stack = grid_p2[dst - 1].borrow_mut();
            let mut src_stack = grid_p2[src - 1].borrow_mut();
            let src_start = src_stack.len() - count;
            dst_stack.extend(src_stack.drain(src_start..));
        }
    }

    let p1 = extract_tops(grid_p1);
    let p2 = extract_tops(grid_p2);
    println!("{}\n{}", p1, p2);
}

fn extract_tops(grid_p1: Vec<RefCell<Vec<u8>>>) -> String {
    String::from_utf8(
        grid_p1
            .iter()
            .map(|stack| *stack.borrow().last().unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn make_stacks(lines: impl Iterator<Item = String>) -> Vec<RefCell<Vec<u8>>> {
    let mut stacks_transposed = lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.as_bytes()
                .chunks(4)
                .map(|chunk| match &chunk[0..3] {
                    &[b' ', b' ', b' '] => None,
                    &[b'[', c, b']'] => Some(c),
                    &[b' ', _n, b' '] => None, // Footer
                    other => unreachable!("{:?}", std::str::from_utf8(other)),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let footer = stacks_transposed.pop().unwrap();
    let stacks = vec![RefCell::new(Vec::new()); footer.len()];
    for row in stacks_transposed.into_iter().rev() {
        for (i, c) in row.into_iter().enumerate() {
            if let Some(c) = c {
                stacks[i].borrow_mut().push(c);
            }
        }
    }
    stacks
}
