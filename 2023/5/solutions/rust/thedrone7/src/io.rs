use std::io::{self, stdout, BufRead, Write};

pub fn read_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        input.push_str(&line);
        input.push_str("\n")
    }

    input.clone()
}

pub fn write_output(output: String) {
	stdout()
		.write_all(output.as_bytes())
		.unwrap();
}