use std::{convert::Infallible, ops::Range, str::FromStr};

#[derive(Debug)]
struct Mapping {
    source: Range<i64>,
    shift: i64,
}

impl Mapping {
    fn map_seed(&self, seed: i64) -> Option<i64> {
        if self.source.contains(&seed) {
            Some(seed + self.shift)
        } else {
            None
        }
    }

    fn map_range_mapping(
        &self,
        range: &Range<i64>,
    ) -> Option<(Option<Range<i64>>, Range<i64>, Option<Range<i64>>)> {
        Self::intersection(range, &self.source).map(|intersection| {
            let intersection_end = intersection.end;
            (
                if range.start < intersection.start {
                    Some(range.start..intersection.start)
                } else {
                    None
                },
                intersection.start + self.shift..intersection.end + self.shift,
                if intersection_end < range.end {
                    Some(intersection_end..range.end)
                } else {
                    None
                },
            )
        })
    }

    fn intersection(a: &Range<i64>, b: &Range<i64>) -> Option<Range<i64>> {
        let max = i64::max(a.start, b.start);
        let min = i64::min(a.end, b.end);
        if max < min {
            Some(max..min)
        } else {
            None
        }
    }

    fn reverse_mapping(&self) -> Self {
        Self {
            source: self.source.start + self.shift..self.source.end + self.shift,
            shift: -self.shift,
        }
    }
}

impl FromStr for Mapping {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(3, ' ');

        let destination = split.next().unwrap().parse::<i64>().unwrap();
        let source = split.next().unwrap().parse::<i64>().unwrap();
        let length = split.next().unwrap().parse::<i64>().unwrap();

        Ok(Self {
            source: source..source + length,
            shift: destination - source,
        })
    }
}

#[derive(Debug)]
struct Layer {
    mappings: Vec<Mapping>,
}

impl Layer {
    fn map_seed(&self, seed: i64) -> i64 {
        self.mappings
            .iter()
            .find_map(|mapping| mapping.map_seed(seed))
            .unwrap_or(seed)
    }

    fn map_range_layer(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        let (remaining, mut result) = self.mappings.iter().fold(
            (vec![range.clone()], vec![]),
            |(mut ranges, mut done), mapping| {
                let mut next_round = Vec::new();
                while let Some(range) = ranges.pop() {
                    if let Some((remainder_start, mapped_range, remainder_end)) =
                        mapping.map_range_mapping(&range)
                    {
                        done.push(mapped_range);

                        if let Some(remainder_start) = remainder_start {
                            ranges.push(remainder_start);
                        }

                        if let Some(remainder_end) = remainder_end {
                            ranges.push(remainder_end);
                        }
                    } else {
                        next_round.push(range);
                    }
                }

                // swap
                (next_round, done)
            },
        );

        result.extend(remaining);

        result
    }
}

impl FromStr for Layer {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            mappings: s
                .split_once('\n')
                .unwrap()
                .1
                .lines()
                .map(Mapping::from_str)
                .map(Result::unwrap)
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct Almanac {
    layers: [Layer; 7],
}

impl Almanac {
    pub fn map_seed(&self, seed: i64) -> i64 {
        self.layers
            .iter()
            .fold(seed, |seed, layer| layer.map_seed(seed))
    }

    pub fn map_range(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        self.layers
            .iter()
            .fold(vec![range.clone()], |ranges, layer| {
                ranges
                    .iter()
                    .flat_map(|range| layer.map_range_layer(range))
                    .collect()
            })
    }
}

impl FromStr for Almanac {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            layers: s
                .split("\n\n")
                .map(Layer::from_str)
                .map(Result::unwrap)
                .collect::<Vec<Layer>>()
                .try_into()
                .unwrap(),
        })
    }
}
