mod io;

#[macro_use] extern crate scan_fmt;
use itertools::Itertools;
use io::{read_input, write_output};

fn main() {
	let input = read_input();
	let contents = input.lines();

	let mut total_score = 0;
	let mut card_counts: Vec<u32> = Vec::new();

	let count = contents.clone().count();

	for (i, line) in contents.enumerate() {
		if i < card_counts.len() {
			card_counts[i] += 1;
		} else {
			card_counts.push(1);
		}

		let current_count = card_counts[i];

		let (winners, hand) = scan_fmt_some!(
			line.into(), 
			"Card {*d}: {[0-9 ]} | {[0-9 ]}", 
			String, String
		);

		let winners = winners.unwrap();
		let hand = hand.unwrap();

		let winning_numbers = winners.split(" ").filter_map(|h| {
			if h != "" {
				Some(h.parse::<i32>().unwrap())
			} else {
				None
			}
		}).collect_vec();
		let hand_numbers = hand.split(" ").filter_map(|h| {
			if h != "" {
				Some(h.parse::<i32>().unwrap())
			} else {
				None
			}
		}).collect_vec();

		let mut score: usize = 0;

		for card in hand_numbers {
			if winning_numbers.contains(&card) {
				if score == 0 {
					score = 1;
				} else {
					score = score << 1;
				}
			}
		}

		// Part 1
		total_score += score;

		// Part 2
		let mut wins = 0;
		if score > 0 {
			wins = score.ilog2();
		}

		if score > 0 {
			for a in i+1..(i + 2 + (wins as usize)).min(count) as usize {
				if a >= card_counts.len() {
					card_counts.push(current_count);
				} else {
					card_counts[a] += current_count;
				}
			}
		}
	}

	let total_cards: u32 = card_counts.iter().sum();
	let output = format!("{}\n{}\n", total_score, total_cards);
	write_output(output);
}
