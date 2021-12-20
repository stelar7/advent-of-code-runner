use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

const DIMENSIONS: usize = 3;
const SCANNER_RANGE: i32 = 1000;

const FEATURE_SIZE: usize = 3;
const FEATURE_RANGE: i32 = SCANNER_RANGE / 4;

const OVERLAP_THRESHOLD: usize = 12;

type Coord = [i32; DIMENSIONS];
type Feature = [Coord; FEATURE_SIZE];

type Rotation = [(usize, bool); 3];
type Translation = [i32; DIMENSIONS];

const ROTATIONS: [Rotation; 24] = [
    [(0, false), (1, false), (2, false)],
    [(1, true), (0, false), (2, false)],
    [(0, true), (1, true), (2, false)],
    [(1, false), (0, true), (2, false)],
    [(1, false), (2, false), (0, false)],
    [(2, true), (1, false), (0, false)],
    [(1, true), (2, true), (0, false)],
    [(2, false), (1, true), (0, false)],
    [(2, false), (0, false), (1, false)],
    [(0, true), (2, false), (1, false)],
    [(2, true), (0, true), (1, false)],
    [(0, false), (2, true), (1, false)],
    [(1, true), (0, true), (2, true)],
    [(0, false), (1, true), (2, true)],
    [(1, false), (0, false), (2, true)],
    [(0, true), (1, false), (2, true)],
    [(0, true), (2, true), (1, true)],
    [(2, false), (0, true), (1, true)],
    [(0, false), (2, false), (1, true)],
    [(2, true), (0, false), (1, true)],
    [(2, true), (1, true), (0, true)],
    [(1, false), (2, true), (0, true)],
    [(2, false), (1, false), (0, true)],
    [(1, true), (2, false), (0, true)],
];

fn apply_rotation(point: Coord, rotation: &Rotation) -> Coord {
    let &[(xi, xn), (yi, yn), (zi, zn)] = rotation;
    [
        if xn { -point[xi] } else { point[xi] },
        if yn { -point[yi] } else { point[yi] },
        if zn { -point[zi] } else { point[zi] },
    ]
}
fn unapply_rotation(point: Coord, rotation: &Rotation) -> Coord {
    let &[(xi, xn), (yi, yn), (zi, zn)] = rotation;
    let mut out = [0; 3];
    out[xi] = if xn { -point[0] } else { point[0] };
    out[yi] = if yn { -point[1] } else { point[1] };
    out[zi] = if zn { -point[2] } else { point[2] };
    out
}

fn apply_translation(point: &mut Coord, translation: &Translation) {
    point[0] += translation[0];
    point[1] += translation[1];
    point[2] += translation[2];
}
fn unapply_translation(point: &mut Coord, translation: &Translation) {
    point[0] -= translation[0];
    point[1] -= translation[1];
    point[2] -= translation[2];
}

fn get_offset_translation(points: &[Coord]) -> Translation {
    let mut offset_translation = [i32::MAX; DIMENSIONS];
    for point in points {
        offset_translation[0] = offset_translation[0].min(point[0]);
        offset_translation[1] = offset_translation[1].min(point[1]);
        offset_translation[2] = offset_translation[2].min(point[2]);
    }
    offset_translation
}

fn find_features(
    points: &[Coord],
    mut visit: impl FnMut((Feature, Rotation, Translation)),
) -> usize {
    let mut count = 0;
    'outer: for candidate_points in points.iter().cloned().permutations(FEATURE_SIZE) {
        for i in 0..3 {
            let min = candidate_points.iter().map(|p| p[i]).min().unwrap();
            let max = candidate_points.iter().map(|p| p[i]).max().unwrap();
            if FEATURE_RANGE < max - min {
                continue 'outer;
            }
        }
        let mut drain = candidate_points.into_iter();
        let uncanon_feature = [(); FEATURE_SIZE].map(|_| drain.next().unwrap());
        (visit)(canonicalize_feature(&uncanon_feature));
        assert!(drain.next().is_none());
        count += 1;
    }
    count
}

fn canonicalize_feature(feature: &Feature) -> (Feature, Rotation, Translation) {
    ROTATIONS
        .iter()
        .map(|&rotation| {
            let mut new_feature = feature.map(|point| apply_rotation(point, &rotation));
            let offset_translation = get_offset_translation(&new_feature);
            for point in &mut new_feature {
                unapply_translation(point, &offset_translation);
            }
            (new_feature, rotation, offset_translation)
        })
        .min_by_key(|&(feature, _rotation, _offset_translation)| feature)
        .unwrap()
}

fn get_matching_coords<'a>(
    scanner_a: &'a [Coord],
    rotate_a: &'a Rotation,
    offset_translation_a: &'a Translation,
    scanner_b: &'a [Coord],
    rotate_b: &'a Rotation,
    offset_translation_b: &'a Translation,
) -> impl 'a + Iterator<Item = Coord> {
    scanner_b
        .iter()
        .map(|&coord| apply_rotation(coord, rotate_b))
        .map(|mut coord| {
            unapply_translation(&mut coord, offset_translation_b);
            coord
        })
        .map(|mut coord| {
            apply_translation(&mut coord, offset_translation_a);
            coord
        })
        .map(|coord| unapply_rotation(coord, rotate_a))
        .filter(|coord| scanner_a.contains(coord))
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .peekable();

    let mut scanners: Vec<Vec<Coord>> = Default::default();
    while lines.peek().is_some() {
        let scanner = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut nums = line.split(',').map(|n| {
                    n.parse()
                        .unwrap_or_else(|err| panic!("Failed to parse integer \"{}\": {}", n, err))
                });
                let coord = [
                    nums.next().unwrap(),
                    nums.next().unwrap(),
                    nums.next().unwrap(),
                ];
                assert!(nums.next().is_none());
                coord
            })
            .collect();
        scanners.push(scanner);
    }
    let scanners = scanners;

    // let mut features: Vec<Feature> = Default::default();
    let mut features_dict: BTreeMap<Feature, Vec<(usize, Rotation, Translation)>> =
        Default::default();
    for (scanner_idx, scanner) in scanners.iter().enumerate() {
        let count_features = find_features(&*scanner, |(feature, rotation, offset_translation)| {
            features_dict.entry(feature).or_default().push((
                scanner_idx,
                rotation,
                offset_translation,
            ))
        });
        let _ = count_features;
        // println!("scanner_idx={}, features={}", scanner_idx, count_features);
    }
    features_dict.retain(|_k, v| 1 < v.len());

    // for (i, matches) in features_dict.values().enumerate() {
    //     println!(
    //         "{}: {} -> {:?}",
    //         i,
    //         matches.len(),
    //         matches
    //             .iter()
    //             .map(|&(scanner_idx, _, _)| scanner_idx)
    //             .collect::<Vec<_>>()
    //     );
    // }

    let mut matched_scanners: BTreeMap<usize, Vec<(Rotation, Translation, Rotation, Translation)>> =
        Default::default();
    matched_scanners.insert(0, vec![]);

    while matched_scanners.len() < scanners.len() {
        for scanners_by_feature in features_dict.values() {
            for pair in scanners_by_feature.iter().permutations(2) {
                if let [match_a, match_b] = &*pair {
                    let (scanner_idx_a, rotate_a, offset_translation_a) = *match_a;
                    let (scanner_idx_b, rotate_b, offset_translation_b) = *match_b;

                    if let (Some(prev_transform), None) = (
                        matched_scanners.get(scanner_idx_a),
                        matched_scanners.get(scanner_idx_b),
                    ) {
                        let scanner_a = &*scanners[*scanner_idx_a];
                        let scanner_b = &*scanners[*scanner_idx_b];

                        let matched_coords = get_matching_coords(
                            scanner_a,
                            rotate_a,
                            offset_translation_a,
                            scanner_b,
                            rotate_b,
                            offset_translation_b,
                        );
                        // .collect::<Vec<_>>();
                        if OVERLAP_THRESHOLD <= matched_coords.count() {
                            // println!("Matched! {} {}", scanner_idx_a, scanner_idx_b);
                            // println!("{:?}", matched_coords);

                            let mut new_transform = prev_transform.clone();
                            new_transform.push((
                                *rotate_a,
                                *offset_translation_a,
                                *rotate_b,
                                *offset_translation_b,
                            ));
                            matched_scanners.insert(*scanner_idx_b, new_transform);
                        }
                    }
                } else {
                    unreachable!();
                }
            }
        }
    }

    // println!("{:?}", matched_scanners);

    let mut points: BTreeSet<Coord> = Default::default();
    let mut scanner_positions: Vec<Coord> = Default::default();

    for (scanner_idx, transform) in matched_scanners {
        let transformed_scanner_points = scanners[scanner_idx].iter().cloned().map(|mut coord| {
            for (rotate_a, offset_translation_a, rotate_b, offset_translation_b) in
                transform.iter().rev()
            {
                coord = apply_rotation(coord, rotate_b);
                unapply_translation(&mut coord, offset_translation_b);
                apply_translation(&mut coord, offset_translation_a);
                coord = unapply_rotation(coord, rotate_a);
            }
            coord
        });
        points.extend(transformed_scanner_points);

        let mut scanner_position = [0; DIMENSIONS];
        for (rotate_a, offset_translation_a, rotate_b, offset_translation_b) in
            transform.iter().rev()
        {
            scanner_position = apply_rotation(scanner_position, rotate_b);
            unapply_translation(&mut scanner_position, offset_translation_b);
            apply_translation(&mut scanner_position, offset_translation_a);
            scanner_position = unapply_rotation(scanner_position, rotate_a);
        }
        scanner_positions.push(scanner_position);
    }

    // println!("POINTS");
    // for point in &points {
    //     println!("{},{},{}", point[0], point[1], point[2]);
    // }

    let max_scanner_dist = scanner_positions
        .into_iter()
        .permutations(2)
        .map(|pair| {
            if let [a, b] = &*pair {
                (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
            } else {
                unreachable!();
            }
        })
        .max()
        .unwrap();

    println!("{}\n{}", points.len(), max_scanner_dist);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotation() {
        let x = [1, 2, 3];
        for rotation in &ROTATIONS {
            assert_eq!(x, unapply_rotation(apply_rotation(x, rotation), rotation));
            assert_eq!(x, apply_rotation(unapply_rotation(x, rotation), rotation));
        }
    }
}
