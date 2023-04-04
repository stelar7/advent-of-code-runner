use std::{cmp::Reverse, io::BufRead, str::FromStr};

use itertools::Itertools;

pub struct Monkey {
    op: Box<dyn Fn(usize) -> usize>,
    test: usize,
    t: usize,
    f: usize,
}

pub fn main() {
    let mut item_queues = Vec::new();

    let monkeys: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .filter(|line| !line.is_empty())
        .tuples()
        .enumerate()
        .map(
            |(i, (monkey_str, items_str, op_str, test_str, t_str, f_str))| {
                debug_assert_eq!(
                    i,
                    monkey_str
                        .strip_prefix("Monkey ")
                        .unwrap()
                        .strip_suffix(':')
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                );
                let items: Vec<_> = items_str
                    .strip_prefix("  Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(usize::from_str)
                    .map(Result::unwrap)
                    .collect();
                let op: Box<dyn Fn(usize) -> usize> = match op_str
                    .strip_prefix("  Operation: new = ")
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                {
                    ("old *", "old") => Box::new(|old| old * old),
                    ("old *", n) => {
                        let n = n.parse::<usize>().unwrap();
                        Box::new(move |old| old * n)
                    }
                    ("old +", n) => {
                        let n = n.parse::<usize>().unwrap();
                        Box::new(move |old| old + n)
                    }
                    _ => panic!(),
                };
                let test = test_str
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let t = t_str
                    .strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
                let f = f_str
                    .strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();

                assert_eq!(i, item_queues.len());
                item_queues.push(items);
                Monkey { op, test, t, f }
            },
        )
        .collect();

    assert_eq!(monkeys.len(), item_queues.len());

    let p1 = monkey_around::<20, 3>(&*monkeys, item_queues.clone());
    let p2 = monkey_around::<10000, 1>(&*monkeys, item_queues.clone());

    println!("{}\n{}", p1, p2);
}

pub fn monkey_around<const RT: usize, const DIV: usize>(
    monkeys: &[Monkey],
    mut item_queues: Vec<Vec<usize>>,
) -> usize {
    let multiple: usize = monkeys.iter().map(|monkey| monkey.test).product();
    let mut inspections = vec![0; monkeys.len()];
    let mut current_items = Vec::new();
    for _ in 0..RT {
        for (i, monkey) in monkeys.iter().enumerate() {
            std::mem::swap(&mut current_items, &mut item_queues[i]);
            inspections[i] += current_items.len();
            for item in current_items.drain(..) {
                let new_item = (monkey.op)(item) / DIV;
                let new_item = new_item % multiple;
                if 0 == new_item % monkey.test {
                    item_queues[monkey.t].push(new_item);
                } else {
                    item_queues[monkey.f].push(new_item);
                }
            }
        }
    }
    inspections.sort_unstable_by_key(|&x| Reverse(x));
    inspections[0] * inspections[1]
}
