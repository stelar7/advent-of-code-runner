use std::collections::HashMap;

const SPACES: usize = 10;
const GOAL_1: usize = 1000;
const GOAL_2: usize = 21;

fn play_single(mut positions: [usize; 2], goal: usize, mut dice: impl FnMut() -> usize) -> usize {
    let mut scores = [0_usize; 2];
    let mut turns = 0;
    loop {
        for player in 0..2 {
            let roll = (dice)() + (dice)() + (dice)();
            turns += 1;

            positions[player] = (positions[player] + roll) % (SPACES as usize);
            scores[player] += positions[player] + 1;
            if goal <= scores[player] {
                return scores[1 - player] * turns * 3;
            }
        }
    }
}

const ROLL_DISTRIBUTION: [u64; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

fn play_quantum(
    cache: &mut HashMap<(usize, usize, usize, usize), (u64, u64)>,
    pos_a: usize,
    pos_b: usize,
    score_a: usize,
    score_b: usize,
) -> (u64, u64) {
    let cache_key = (pos_a, pos_b, score_a, score_b);
    if GOAL_2 <= score_a {
        (1, 0)
    } else if GOAL_2 <= score_b {
        (0, 1)
    } else if let Some(cached_output) = cache.get(&cache_key) {
        *cached_output
    } else {
        let mut output = (0, 0);
        for roll in 3..=9 {
            let mult = ROLL_DISTRIBUTION[roll];
            let next_pos_a = (pos_a + roll) % SPACES;

            let next = play_quantum(cache, pos_b, next_pos_a, score_b, score_a + next_pos_a + 1);
            output.0 += mult * next.1;
            output.1 += mult * next.0;
        }
        cache.insert(cache_key, output);
        output
    }
}

fn main() {
    let mut part_1_table = [[0_u32; SPACES]; SPACES];
    let mut part_2_table = [[0_u64; SPACES]; SPACES];

    let mut cache = Default::default();
    for pos_a in 0..10 {
        for pos_b in 0..10 {
            part_1_table[pos_a][pos_b] = {
                let mut dice_state = 0;
                play_single([pos_a, pos_b], GOAL_1, || {
                    dice_state %= 100;
                    dice_state += 1;
                    dice_state
                }) as u32
            };
            let part_2_solution = play_quantum(&mut cache, pos_a, pos_b, 0, 0);
            part_2_table[pos_a][pos_b] = std::cmp::max(part_2_solution.0, part_2_solution.1);
        }
    }

    let part_1_bytes: &[u8; 10 * 10 * 4] = unsafe { std::mem::transmute(&part_1_table) };
    let part_2_bytes: &[u8; 10 * 10 * 8] = unsafe { std::mem::transmute(&part_2_table) };

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let file_1: std::path::PathBuf = [&*out_dir, "part_1.bin"].iter().collect();
    let file_2: std::path::PathBuf = [&*out_dir, "part_2.bin"].iter().collect();
    std::fs::write(&file_1, part_1_bytes).unwrap();
    std::fs::write(&file_2, part_2_bytes).unwrap();
}
