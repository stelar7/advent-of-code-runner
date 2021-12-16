use std::io::BufRead;

pub fn fold_bools() -> impl Fn(usize, bool) -> usize {
    |a: usize, b: bool| (a << 1) | (b as usize)
}

pub fn parse_packet<I>(bits: &mut I, checksum: &mut usize) -> usize
where
    I: ?Sized + Iterator<Item = bool>,
{
    let packet_version = bits.take(3).fold(0, fold_bools());
    *checksum += packet_version;

    let packet_type_id = bits.take(3).fold(0, fold_bools());

    if 4 == packet_type_id {
        let mut more = true;
        let mut val = 0;
        while more {
            more = bits.next().unwrap();
            val = bits.take(4).fold(val, fold_bools());
        }
        val
    } else {
        let packet_length_type_id = bits.next().unwrap();
        let vals = match packet_length_type_id {
            // Mode 0: 15 bits => length in bits.
            false => {
                let bit_len = bits.take(15).fold(0, fold_bools());

                // Truncate bits.
                let mut bits_nested = bits.take(bit_len).peekable();

                let mut vals = Vec::new();
                while bits_nested.peek().is_some() {
                    // `dyn` to break recursive type explosion.
                    let bits_dyn: &mut dyn Iterator<Item = bool> = &mut bits_nested;
                    vals.push(parse_packet(bits_dyn, checksum))
                }
                vals
            }
            // Mode 1: 11 bits => num packets.
            true => {
                let num_packets = bits.take(11).fold(0, fold_bools());

                (0..num_packets)
                    .map(|_n| parse_packet(bits, checksum))
                    .collect()
            }
        };

        match packet_type_id {
            0 => vals.into_iter().sum::<usize>(),
            1 => vals.into_iter().product::<usize>(),
            2 => vals.into_iter().min().unwrap(),
            3 => vals.into_iter().max().unwrap(),
            5 => (vals[0] > vals[1]) as usize,
            6 => (vals[0] < vals[1]) as usize,
            7 => (vals[0] == vals[1]) as usize,
            x => panic!("Unknown packet_type_id={}", x),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut bits = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .flat_map(|line| line.into_bytes())
        .map(|c| if b'A' <= c { 10 + c - b'A' } else { c - b'0' })
        .flat_map(|b| [b & 0b1000, b & 0b0100, b & 0b0010, b & 0b0001].map(|b| 0 != b));

    let mut checksum = 0;
    let val = parse_packet(&mut bits, &mut checksum);

    println!("{}\n{}", checksum, val);
}
