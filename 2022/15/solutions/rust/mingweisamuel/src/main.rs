use std::{collections::HashSet, io::BufRead};

use itertools::Itertools;
use scan_fmt::scan_fmt;

const ROW: isize = 2_000_000;

const RANGE: isize = 4_000_000;
const CORNERS: [(isize, isize); 4] = [(0, 0), (0, RANGE), (RANGE, 0), (RANGE, RANGE)];

fn row_range(&(sx, sy, radius): &(isize, isize, usize)) -> Option<(isize, isize)> {
    let shrink = isize::abs_diff(sy, ROW);
    let row_radius = radius.checked_sub(shrink)? as isize;
    Some((sx - row_radius, sx + row_radius))
}

pub fn main() {
    let mut row_beacons = HashSet::new();

    let mut scans: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            scan_fmt!(
                &*line,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                isize,
                isize,
                isize,
                isize
            )
            .unwrap()
        })
        .inspect(|&(_sx, _sy, bx, by)| {
            if ROW == by {
                row_beacons.insert(bx);
            }
        })
        .map(|(sx, sy, bx, by)| {
            let radius = isize::abs_diff(sx, bx) + isize::abs_diff(sy, by);
            (sx, sy, radius)
        })
        .collect();

    // // let y_range = 0_isize..=20;
    // // let x_range = -5_isize..=25;

    // let y_range = 1_999_990_isize..=2_000_010;
    // let x_range = 1_999_990_isize..=2_000_010;

    // for y in y_range {
    //     print!("{:3} ", y);
    //     for x in x_range.clone() {
    //         print!(
    //             "{}",
    //             scans
    //                 .iter()
    //                 .enumerate()
    //                 .filter(|&(_, &(sx, sy, radius))| {
    //                     let d = x.abs_diff(sx) + y.abs_diff(sy);
    //                     d <= radius
    //                 })
    //                 .map(|(i, _)| (b'A' + (i as u8)) as char)
    //                 .next()
    //                 .unwrap_or('.')
    //         );
    //     }
    //     println!();
    // }

    let p1 = {
        scans.sort_unstable_by_key(row_range);
        let sum: usize = scans
            .iter()
            .filter_map(row_range)
            .coalesce(|a, b| {
                if b.0 <= a.1 + 1 {
                    // Overlap or touching!
                    Ok((isize::min(a.0, b.0), isize::max(a.1, b.1)))
                } else {
                    Err((a, b))
                }
            })
            .map(|a| (a.1 - a.0 + 1) as usize)
            .sum();
        sum - row_beacons.len()
    };

    let p2 = {
        let pos_diags = scans
            .iter()
            .flat_map(|&(sx, sy, radius)| {
                [
                    sy - sx - (radius as isize) - 2,
                    sy - sx - (radius as isize) - 1,
                    sy - sx + (radius as isize) + 1,
                    sy - sx + (radius as isize) + 2,
                ]
            })
            .counts();
        let neg_diags = scans
            .iter()
            .flat_map(|&(sx, sy, radius)| {
                [
                    sy + sx - (radius as isize) - 2,
                    sy + sx - (radius as isize) - 1,
                    sy + sx + (radius as isize) + 1,
                    sy + sx + (radius as isize) + 2,
                ]
            })
            .counts();

        let pos_multis = pos_diags
            .iter()
            .filter(|(_y, &count)| 1 < count)
            .map(|(&y, _count)| y);
        let neg_multis = neg_diags
            .iter()
            .filter(|(_y, &count)| 1 < count)
            .map(|(&y, _count)| y);

        let candidates = pos_multis
            .cartesian_product(neg_multis)
            .filter_map(|(pos_diag, neg_diag)| {
                let d = neg_diag - pos_diag;
                if 0 != d % 2 {
                    return None;
                }
                let x = d / 2;
                let y = neg_diag - x;
                Some((x, y))
            })
            .filter(|(x, y)| (0..=RANGE).contains(x) && (0..=RANGE).contains(y))
            .chain(CORNERS);

        let mut beacons: Vec<_> = candidates
            .filter(|(x, y)| {
                scans.iter().all(|&(sx, sy, radius)| {
                    let d = x.abs_diff(sx) + y.abs_diff(sy);
                    radius < d
                })
            })
            .collect();
        beacons.sort_unstable();

        // #[cfg(debug_assertions)]
        // dbg!(&beacons);
        let &(bx, by) = beacons.get(0).expect("Failed to find distress beacon gap.");

        bx * RANGE + by
    };

    println!("{}\n{}", p1, p2);
}
