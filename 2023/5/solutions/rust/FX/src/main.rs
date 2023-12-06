use std::{io::stdin, ops::Range, str::FromStr};

use crate::almanac::Almanac;

mod almanac;

fn main() {
    let input = std::io::read_to_string(stdin().lock()).unwrap();

    let (first_line, layers) = input.split_once("\n\n").unwrap();
    let (_, seeds_str) = first_line.split_once(' ').unwrap();

    let seeds: Vec<i64> = seeds_str
        .split(' ')
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let seeds_ranges: Vec<Range<i64>> = seeds
        .chunks(2)
        .map(|slice| slice[0]..slice[0] + slice[1])
        .collect();

    let almanac = Almanac::from_str(layers).unwrap();

    let results1 = seeds
        .iter()
        .map(|&seed| almanac.map_seed(seed))
        .min()
        .unwrap();
    let results2 = seeds_ranges
        .iter()
        .flat_map(|seed_range| almanac.map_range(seed_range))
        .min_by(|r1, r2| r1.start.cmp(&r2.start))
        .map(|range| range.start)
        .unwrap();

    println!("{results1:?}\n{results2:?}");
}
