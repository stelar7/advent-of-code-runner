mod io;

use itertools::Itertools;
use io::{read_input, write_output};

fn main() {
	let contents = read_input();
	let lines = contents.lines().collect_vec();

	let mut numbers = Vec::new();
	let mut valid_numbers: Vec<(usize, usize, usize, i32)> = Vec::new();
	let mut gear_ratios: Vec<i32> = Vec::new();

	for i in 0..lines.len() {
		let line = lines[i];
		let mut number_str = String::new();
		let mut number_start = 0;

		for (j, c) in line.chars().enumerate() {
			if c.is_digit(10) {
				if number_str.is_empty() {
					number_start = j;
				}
				number_str.push(c);
			} else {
				if !number_str.is_empty() {
					let end = j - 1;
					numbers.push((number_start, end, number_str.clone()));
					number_str.clear();
				}
			}
		}

		
		if !number_str.is_empty() {
			numbers.push((number_start, line.len() - 1, number_str.clone()));
			number_str.clear();
		}

		for number in numbers.iter() {
			let (start, end, number) = number;
			let mut valid = false;

			// Check left
			if start > &0 {
				let left = line.chars().nth(start-1).unwrap();
				if left != '.' && !left.is_digit(10) {
					valid = true;
				}
			}

			// Check right
			if end < &(line.len() - 1) {
				let right = line.chars().nth(end+1).unwrap();
				if right != '.' && !right.is_digit(10) {
					valid = true;
				}
			}

			// Check up
			if i > 0 {
				let up = &lines[i-1].to_string()[*start..*end+1];
				if up.contains(|c: char| c != '.' && !c.is_digit(10)) {
					valid = true;
				}
			}

			// Check down
			if i < lines.len() - 1 {
				let down = &lines[i+1].to_string()[*start..*end+1];
				if down.contains(|c: char| c != '.' && !c.is_digit(10)) {
					valid = true;
				}
			}

			// Check diagonally
			if i > 0 && start > &0 {
				let up_left = lines[i-1].chars().nth(start-1).unwrap();
				if up_left != '.' && !up_left.is_digit(10) {
					valid = true;
				}
			}

			if i > 0 && end < &(lines[i].len() - 1) {
				let up_right = lines[i-1].chars().nth(end+1).unwrap();
				if up_right != '.' && !up_right.is_digit(10) {
					valid = true;
				}
			}

			if i < lines.len() - 1 && start > &0 {
				let down_left = lines[i+1].chars().nth(start-1).unwrap();
				if down_left != '.' && !down_left.is_digit(10) {
					valid = true;
				}
			}

			if i < lines.len() - 1 && end < &(lines[i].len() - 1) {
				let down_right = lines[i+1].chars().nth(end+1).unwrap();
				if down_right != '.' && !down_right.is_digit(10) {
					valid = true;
				}
			}

			if valid {
				valid_numbers.push((i, *start, *end, number.clone().parse::<i32>().unwrap()));
			}
		}

		numbers.clear();
	}

	for (i, line) in lines.iter().enumerate() {
		for (j, c) in line.chars().enumerate() {
			if c == '*' {
				let mut gear_numbers = Vec::new();
				for (line_number, start, end, number) in valid_numbers.iter() {
					if (*line_number) > 0 {
						if (*start) > 0 {
							if i >= (*line_number - 1) && i <= (*line_number + 1) && j >= (*start - 1) && j <= (*end + 1) {
								gear_numbers.push(*number);
							}
						} else if i >= (*line_number - 1) && i <= (*line_number + 1) && j <= (*end + 1) {
							gear_numbers.push(*number);
						}
					} else if (*start) > 0 {
						if i <= (*line_number + 1) && j >= (*start - 1) && j <= (*end + 1) {
							gear_numbers.push(*number);
						}
					} else if  i <= (*line_number + 1) && j <= (*end + 1) {
						gear_numbers.push(*number);
					}
				}

				if gear_numbers.len() == 2 {
					let mut gear_ratio = 1;
					for number in gear_numbers {
						gear_ratio *= number;
					}
					gear_ratios.push(gear_ratio);
				}
			}
		}
	}
	
	let total: i32 = valid_numbers.iter().map(|n| n.3).sum();
	let gear_total: i32 = gear_ratios.iter().sum();

	let output = format!("{}\n{}\n", total, gear_total);
	write_output(output);
}
