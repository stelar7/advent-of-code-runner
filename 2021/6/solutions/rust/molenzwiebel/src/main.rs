use std::io::Read;
use unroll::unroll_for_loops;

#[unroll_for_loops]
fn run(iter: impl Iterator<Item = u8>) -> u64 {
    let mut counts = [0u64; 8];

    // read input
    for inp in iter.step_by(2) {
        counts[(inp - b'0') as usize] += 1;
    }

    // need to keep track of this since we can't just add 9 days
    let mut carry_over = 0;

    for day in 0usize..=256usize {
        let idx_expiring = day % 8;
        let expired_today = counts[idx_expiring];

        counts[idx_expiring] = carry_over;
        counts[(idx_expiring + 7) % 8] += expired_today;

        carry_over = expired_today;

        if day == 80 {
            println!("{:?}", counts.iter().sum::<u64>());
        }
    }

    counts.iter().sum::<u64>()
}

fn main() {
    println!(
        "{}",
        run(std::io::stdin().lock().bytes().map(|x| x.unwrap()))
    );
}
