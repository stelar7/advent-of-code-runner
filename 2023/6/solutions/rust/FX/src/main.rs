use std::io::stdin;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = std::io::read_to_string(stdin().lock())?;

    let (times, distances): (&str, &str) =
        input.split_once('\n').context("error splitting lines")?;

    // part 1
    let records: Vec<Record> = times
        .split_whitespace()
        .skip(1)
        .zip(distances.split_whitespace().skip(1))
        .map(|(time, distance)| Record {
            max_time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect();

    let result1: u64 = records
        .iter()
        .map(|record| margin_of_error(record))
        .product();

    // part 2
    let max_time = times
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut max_time, str| {
            max_time.push_str(str);
            max_time
        })
        .parse()?;

    let distance = distances
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut max_time, str| {
            max_time.push_str(str);
            max_time
        })
        .parse()?;

    let record = Record { max_time, distance };
    let result2 = margin_of_error(&record);

    println!("{result1}\n{result2}");

    Ok(())
}

#[derive(Debug)]
struct Record {
    max_time: f64,
    distance: f64,
}

#[inline(always)]
fn margin_of_error(record: &Record) -> u64 {
    let tmp = (record.max_time.powi(2) - 4.0 * record.distance).sqrt();
    let left_limit = 0.5 * (record.max_time - tmp);
    let right_limit = 0.5 * (record.max_time + tmp);

    if right_limit.floor() == right_limit {
        right_limit.floor() as u64 - left_limit.floor() as u64 - 1
    } else {
        right_limit.floor() as u64 - left_limit.floor() as u64
    }
}
