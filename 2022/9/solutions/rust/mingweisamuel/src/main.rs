use std::{collections::HashSet, io::BufRead};

pub fn follow(head: (i32, i32), tail: &mut (i32, i32)) -> bool {
    let dy = head.0 - tail.0;
    let dx = head.1 - tail.1;
    if dy.abs() <= 1 && dx.abs() <= 1 {
        return false;
    }
    tail.0 += dy.signum();
    tail.1 += dx.signum();
    true
}

pub fn main() {
    // y, x
    let mut rope = [(0_i32, 0_i32); 10];

    let mut visited_1: HashSet<_> = [rope[1]].into_iter().collect();
    let mut visited_9: HashSet<_> = [rope[9]].into_iter().collect();

    for line in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
    {
        let (dir, num) = line.split_once(' ').unwrap();
        let dist = num.parse::<u32>().unwrap() as i32;
        let d = match dir {
            "U" => (1, 0),
            "D" => (-1, 0),
            "R" => (0, 1),
            "L" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..dist {
            rope[0].0 += d.0;
            rope[0].1 += d.1;
            let mut head = rope[0];
            for tail in rope[1..].iter_mut() {
                if !follow(head, tail) {
                    break;
                }
                head = *tail;
            }
            visited_1.insert(rope[1]);
            visited_9.insert(rope[9]);
        }

        // println!("{}", line);
        // for y in (-14..15).rev() {
        //     for x in -14..15 {
        //         if let Some(n) = rope.iter().position(|&yx| yx == (y, x)) {
        //             print!("{}", n);
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!("---");
    }

    println!("{}\n{}", visited_1.len(), visited_9.len());
}
