pub struct RelMap {
    pub destinations: Vec<usize>,
    pub sources: Vec<usize>,
    pub lengths: Vec<usize>,
}

impl RelMap {
    pub fn generate(input: String) -> RelMap {
        let mut destinations = Vec::new();
        let mut sources = Vec::new();
        let mut lengths = Vec::new();

        let mut lines = input.lines();
        lines.next();

        for line in lines {
            let (dest, src, len) = scan_fmt_some!(line, "{} {} {}", usize, usize, usize);
            destinations.push(dest.unwrap());
            sources.push(src.unwrap());
            lengths.push(len.unwrap());
        }

        RelMap {
            destinations,
            sources,
            lengths,
        }
    }

    pub fn get_value_for(&self, value: usize) -> usize {
        let mut result = value;

        for i in 0..self.sources.len() {
            let beginning = self.sources[i];
            let end = self.sources[i] + self.lengths[i];

            if value >= beginning && value < end {
                result = self.destinations[i] + (value - beginning);
                break;
            }
        }

        result
    }

    pub fn get_value_and_next(&self, value: usize) -> (usize, usize) {
        let mut result = value;
        let mut next = usize::MAX;

        for i in 0..self.sources.len() {
            let beginning = self.sources[i];
            let end = self.sources[i] + self.lengths[i];

            if value >= beginning && value < end {
                result = self.destinations[i] + (value - beginning);
                next = self.lengths[i] - (value - beginning);
                break;
            } else if beginning > value {
                next = next.min(beginning - value);
            }
        }

        (result, next)
    }
}

pub fn process_value(value: usize, maps: &Vec<RelMap>) -> usize {
    let mut result = value;

    for map in maps {
        result = map.get_value_for(result);
    }

    result
}

pub fn process_value_and_next(value: usize, maps: &Vec<RelMap>) -> (usize, usize) {
    let mut result = value;
    let mut next = usize::MAX;

    for map in maps {
        let (new_result, new_next) = map.get_value_and_next(result);
        result = new_result;
        next = next.min(new_next);
    }

    (result, next)
}
