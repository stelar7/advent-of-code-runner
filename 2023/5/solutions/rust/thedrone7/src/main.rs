mod io;
mod map;

#[macro_use]
extern crate scan_fmt;

use io::{read_input, write_output};
use itertools::Itertools;
use map::{process_value, process_value_and_next, RelMap};

fn main() {
    let contents = read_input();
    let mut sections = contents.split("\n\n").collect_vec();
    let mut maps = Vec::new();
    let seeds = sections
        .remove(0)
        .get(7..)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    sections.iter().for_each(|section| {
        maps.push(RelMap::generate(section.to_string()));
    });

    let mut lowest = usize::MAX;
    let mut real_lowest = usize::MAX;

    for i in (0..seeds.len()).step_by(2) {
        let first_seed = seeds[i];
        let seeds_count = seeds[i + 1];
        let last_seed = first_seed + seeds_count;

        let mut j = first_seed;
        while j < last_seed {
            let (result, next) = process_value_and_next(j, &maps);
            real_lowest = real_lowest.min(result);
            if j == first_seed {
                lowest = lowest.min(result);
            }

            j += next;
        }

        lowest = lowest.min(process_value(seeds_count, &maps));
    }

    let output = format!("{}\n{}\n", lowest, real_lowest);
    write_output(output);
}
