use std::io::BufRead;

fn main() {
    let (p1, p2) = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let (r0, r1) = line.split_once(',').unwrap();
            let (a0, a1) = r0.split_once('-').unwrap();
            let (b0, b1) = r1.split_once('-').unwrap();
            let [a0, a1, b0, b1]: [u32; 4] = [a0, a1, b0, b1].map(|s| s.parse().unwrap());
            ((a0, a1), (b0, b1))
        })
        .map(|((a0, a1), (b0, b1))| {
            let p1 = (a0 <= b0 && b1 <= a1) || (b0 <= a0 && a1 <= b1);
            let p2 = p1 || (a0 <= b1 && b0 <= a1);
            (p1, p2)
        })
        .fold((0, 0), |(p1, p2), (x1, x2)| {
            (p1 + x1 as u32, p2 + x2 as u32)
        });
    println!("{}\n{}", p1, p2);
}
