use std::{io::BufRead, iter::Peekable, ops::Deref};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Ord)]
enum Item {
    Value(u8),
    List(Box<[Item]>),
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Value(a), Item::Value(b)) => a.partial_cmp(b),
            (Item::Value(a), Item::List(b)) => [Item::Value(*a)].as_slice().partial_cmp(b.deref()),
            (Item::List(a), Item::Value(b)) => a.deref().partial_cmp([Item::Value(*b)].as_slice()),
            (Item::List(a), Item::List(b)) => a.partial_cmp(b),
        }
    }
}

fn parse_item(cursor: &mut Peekable<impl Iterator<Item = u8>>) -> Item {
    if Some(&b'[') == cursor.peek() {
        let _ = cursor.next();
        let mut list = Vec::new();
        loop {
            match cursor.peek().unwrap() {
                &b']' => {
                    let _ = cursor.next();
                    break Item::List(list.into_boxed_slice());
                }
                &b',' => {
                    let _ = cursor.next();
                }
                _ => list.push(parse_item(cursor)),
            }
        }
    } else {
        let mut val = 0;
        while cursor.peek().map_or(false, |c| c.is_ascii_digit()) {
            val *= 10;
            val += cursor.next().unwrap() - b'0';
        }
        Item::Value(val)
    }
}

pub fn main() {
    let items: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .filter(|line| !line.is_empty())
        .map(|line| parse_item(line.as_bytes().iter().copied().peekable().by_ref()))
        .collect();
    let p1: usize = items
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_idx, (left, rght))| left < rght)
        .map(|(idx, _)| idx + 1)
        .sum();

    let d2 = Item::List([Item::List([Item::Value(2)].into())].into());
    let d6 = Item::List([Item::List([Item::Value(6)].into())].into());
    let mut items = items;
    items.sort_unstable_by_key(|item| &d2 <= item);
    items.sort_by_key(|item| &d6 <= item);
    let i2 = items.partition_point(|item| item < &d2);
    let i6 = items.partition_point(|item| item < &d6);
    let p2 = (i2 + 1) * (i6 + 2);

    println!("{}\n{}", p1, p2);
}
