use std::io::BufRead;

/*
   PART 1:

   OP   R0  P1  S2
   ME
   R0    3   0   6
   P1    6   3   0
   S2    0   6   3


   PART 2 shape:

   OP   R0  P1  S2
   ME
   L0    3   1   2
   D1    1   2   3
   W2    2   1   3
*/

fn main() {
    let (p1, p2) = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line as UTF-8.");
            if let &[a, b' ', x] = line.as_bytes() {
                [a - b'A', x - b'X']
            } else {
                panic!();
            }
        })
        .map(|[op, me]| {
            let p1 = 1 + (me as u32) + 3 * (((4 + me - op) as u32) % 3);
            let p2 = 3 * (me as u32) + 1 + ((2 + me + op) as u32) % 3;
            (p1, p2)
        })
        .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));
    println!("{}\n{}", p1, p2);
}
