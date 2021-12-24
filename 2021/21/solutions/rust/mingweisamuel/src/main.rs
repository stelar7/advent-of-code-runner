use scan_fmt::scan_fmt;
use std::io::BufRead;

const SPACES: usize = 10;
const PLAYERS: usize = 2;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let (_, pos_1) =
                scan_fmt!(&*line, "Player {d} starting position: {d}", usize, usize).unwrap();
            pos_1 - 1
        });

    let start_positions = [(); PLAYERS].map(|_| lines.next().unwrap());
    assert!(lines.next().is_none());

    let part_1_bytes = std::include_bytes!(concat!(env!("OUT_DIR"), "/part_1.bin"));
    let part_1_table: &[[u32; SPACES]; SPACES] = unsafe { std::mem::transmute(part_1_bytes) };

    let part_2_bytes = std::include_bytes!(concat!(env!("OUT_DIR"), "/part_2.bin"));
    let part_2_table: &[[u64; SPACES]; SPACES] = unsafe { std::mem::transmute(part_2_bytes) };

    println!(
        "{}\n{}",
        part_1_table[start_positions[0]][start_positions[1]],
        part_2_table[start_positions[0]][start_positions[1]],
    );
}
