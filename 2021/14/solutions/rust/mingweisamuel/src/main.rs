use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;

type Count = HashMap<u8, usize>;

pub struct PolymerSolver {
    rules: HashMap<(u8, u8), u8>,
    cache: HashMap<((u8, u8), usize), Rc<Count>>,
}
impl PolymerSolver {
    pub fn new(rules: HashMap<(u8, u8), u8>) -> Self {
        Self {
            rules,
            cache: Default::default(),
        }
    }

    pub fn solve(&mut self, polymer: &[u8], steps: usize) -> Count {
        let mut count = Count::new();
        for &c in polymer.iter() {
            *count.entry(c).or_default() += 1;
        }
        for pair in polymer.iter().cloned().tuple_windows() {
            Self::merge_count(&mut count, &*self.solve_pair(pair, steps));
        }
        count
    }

    fn solve_pair(&mut self, pair: (u8, u8), steps: usize) -> Rc<Count> {
        let cache_key = (pair, steps);
        if let Some(cached_count) = self.cache.get(&cache_key) {
            return Rc::clone(cached_count);
        }

        let &new_char = self.rules.get(&pair).unwrap();

        let mut count = Count::new();
        *count.entry(new_char).or_default() += 1;
        if 0 < steps {
            Self::merge_count(&mut count, &*self.solve_pair((pair.0, new_char), steps - 1));
            Self::merge_count(&mut count, &*self.solve_pair((new_char, pair.1), steps - 1));
        }

        self.cache.insert(cache_key, Rc::new(count));
        Rc::clone(self.cache.get(&cache_key).unwrap())
    }

    fn merge_count(target: &mut Count, extra: &Count) {
        for (k, v) in extra {
            *target.entry(*k).or_default() += v;
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));

    let polymer = lines.next().unwrap();

    let rules: HashMap<(u8, u8), u8> = lines
        .skip(1)
        .map(|line| {
            let (from, to) = line.split_once(" -> ").expect("Line missing arrow.");
            assert_eq!(2, from.len());
            assert_eq!(1, to.len());
            let from = from.as_bytes();
            let from = (from[0], from[1]);
            let to = to.as_bytes()[0];
            (from, to)
        })
        .collect();

    let mut solver = PolymerSolver::new(rules);

    let count = solver.solve(polymer.as_bytes(), 9);
    let a = count.values().max().unwrap() - count.values().min().unwrap();
    let count = solver.solve(polymer.as_bytes(), 39);
    let b = count.values().max().unwrap() - count.values().min().unwrap();
    println!("{}\n{}", a, b);
}
