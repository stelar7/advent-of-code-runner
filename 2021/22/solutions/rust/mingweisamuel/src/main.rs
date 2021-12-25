use scan_fmt::scan_fmt;
use std::io::BufRead;

type Number = i32;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Range {
    pub min: Number,
    pub max: Number,
}

#[derive(Default, Clone)]
pub struct KdTreeSet<const K: usize> {
    root: Node<K>,
}
impl<const K: usize> KdTreeSet<K> {
    const INFINITE_CUBOID: [Range; K] = [Range {
        min: Number::MIN,
        max: Number::MAX,
    }; K];

    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_value(value: bool) -> Self {
        Self {
            root: Node::Value(value),
        }
    }

    pub fn count_values(&self) -> u64 {
        self.root.count_values(Self::INFINITE_CUBOID)
    }
    pub fn set_cuboid(&mut self, value_cuboid: [Range; K], value: bool) {
        self.root
            .set_cuboid(Self::INFINITE_CUBOID, value_cuboid, value);
    }
}

#[derive(Debug, Clone)]
enum Node<const K: usize> {
    Value(bool),
    Parent {
        // This axis this node splits along.
        axis: usize,
        // The coordinate to split at. This is the included in the SECOND (high) child.
        // I.e. exclusive upper bound for low, inclusive lower bound for high.
        split: Number,
        // The low and high children.
        children: Box<(Node<K>, Node<K>)>,
    },
}
impl<const K: usize> Default for Node<K> {
    fn default() -> Self {
        Self::Value(false)
    }
}
impl<const K: usize> Node<K> {
    fn count_values(&self, node_cuboid: [Range; K]) -> u64 {
        match self {
            &Self::Value(value) => {
                if value {
                    node_cuboid
                        .into_iter()
                        .map(|Range { min, max }| ((max as i64) - (min as i64)) as u64)
                        .product::<u64>()
                } else {
                    0
                }
            }
            Self::Parent {
                axis,
                split,
                children,
            } => {
                let low_count = {
                    let mut low_child_cuboid = node_cuboid;
                    low_child_cuboid[*axis].max = *split;
                    children.0.count_values(low_child_cuboid)
                };
                let hih_count = {
                    let mut hih_child_cuboid = node_cuboid;
                    hih_child_cuboid[*axis].min = *split;
                    children.1.count_values(hih_child_cuboid)
                };
                low_count + hih_count
            }
        }
    }
    fn set_cuboid(&mut self, node_cuboid: [Range; K], value_cuboid: [Range; K], value: bool) {
        match self {
            &mut Self::Value(old_value) => {
                if old_value == value {
                    return;
                }
                for axis in 0..K {
                    let node_range = node_cuboid[axis];
                    let value_range = value_cuboid[axis];

                    // Check if: CURRENT and VALUE do not overlap.
                    let no_overlap =
                        node_range.max <= value_range.min || value_range.max <= node_range.min;
                    if no_overlap {
                        return;
                    }

                    // Check if: VALUE half-intersects CURRENT on the low-end.
                    let intersects_low =
                        node_range.min < value_range.max && value_range.max < node_range.max;
                    if intersects_low {
                        let mut children =
                            Box::new((Self::Value(old_value), Self::Value(old_value)));
                        let split = value_range.max;

                        let mut child_cuboid = node_cuboid;
                        child_cuboid[axis].max = split;
                        children.0.set_cuboid(child_cuboid, value_cuboid, value);

                        *self = Self::Parent {
                            axis,
                            split,
                            children,
                        };
                        return;
                    }
                    // Check if: VALUE half-intersects CURRENT on the high-end.
                    let intersects_hih =
                        node_range.min < value_range.min && value_range.min < node_range.max;
                    if intersects_hih {
                        let mut children =
                            Box::new((Self::Value(old_value), Self::Value(old_value)));
                        let split = value_range.min;

                        let mut child_cuboid = node_cuboid;
                        child_cuboid[axis].min = split;
                        children.1.set_cuboid(child_cuboid, value_cuboid, value);

                        *self = Self::Parent {
                            axis,
                            split,
                            children,
                        };
                        return;
                    }

                    // Else: VALUE completely contains CURRENT. Continue.
                }

                // VALUE completely contains CURRENT in all axis. Update value.
                *self = Self::Value(value);
            }
            Self::Parent {
                axis,
                split,
                children,
            } => {
                {
                    let mut low_child_cuboid = node_cuboid;
                    low_child_cuboid[*axis].max = *split;
                    children.0.set_cuboid(low_child_cuboid, value_cuboid, value);
                }
                {
                    let mut hih_child_cuboid = node_cuboid;
                    hih_child_cuboid[*axis].min = *split;
                    children.1.set_cuboid(hih_child_cuboid, value_cuboid, value);
                }
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let cuboids = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let (on_off, xn, xx, yn, yx, zn, zx) = scan_fmt!(
                &*line,
                "{} x={d}..{d},y={d}..{d},z={d}..{d}",
                String,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            let is_on = match &*on_off {
                "on" => true,
                "off" => false,
                _ => panic!(),
            };
            let xr = Range {
                min: xn,
                max: xx + 1,
            };
            let yr = Range {
                min: yn,
                max: yx + 1,
            };
            let zr = Range {
                min: zn,
                max: zx + 1,
            };
            (is_on, [xr, yr, zr])
        });

    let mut part_a = None;

    let mut kd_tree = KdTreeSet::new();
    for (is_on, cuboid) in cuboids {
        if part_a.is_none()
            && cuboid
                .iter()
                .any(|range| range.max <= -50 || 50 <= range.min)
        {
            part_a.replace(kd_tree.count_values());
        }
        kd_tree.set_cuboid(cuboid, is_on);
    }

    let part_b = kd_tree.count_values();
    println!("{}\n{}", part_a.unwrap_or(part_b), part_b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlap_none() {
        let mut kd_tree = KdTreeSet::new();
        kd_tree.set_cuboid([Range { min: 0, max: 4 }; 2], true);
        kd_tree.set_cuboid([Range { min: 4, max: 8 }; 2], true);
        assert_eq!(32, kd_tree.count_values());
    }

    #[test]
    fn test_overlap_corner() {
        let mut kd_tree = KdTreeSet::new();
        kd_tree.set_cuboid([Range { min: 0, max: 4 }; 2], true);
        kd_tree.set_cuboid([Range { min: 2, max: 6 }; 2], true);
        assert_eq!(28, kd_tree.count_values());
    }

    #[test]
    fn test_overlap_edge_aligned() {
        let mut kd_tree = KdTreeSet::new();
        kd_tree.set_cuboid([Range { min: 0, max: 4 }; 2], true);
        kd_tree.set_cuboid([Range { min: 0, max: 4 }, Range { min: 2, max: 6 }], true);
        assert_eq!(16 + 16 - 8, kd_tree.count_values());
    }

    #[test]
    fn test_overlap_edge_inside() {
        let mut kd_tree = KdTreeSet::new();
        kd_tree.set_cuboid([Range { min: 0, max: 8 }; 2], true);
        kd_tree.set_cuboid([Range { min: 4, max: 12 }, Range { min: 2, max: 6 }], true);
        assert_eq!(64 + 16, kd_tree.count_values());
    }

    #[test]
    fn test_subtract_edge_aligned() {
        let mut kd_tree = KdTreeSet::new();
        kd_tree.set_cuboid([Range { min: 0, max: 4 }; 2], true);
        kd_tree.set_cuboid([Range { min: 0, max: 4 }, Range { min: 2, max: 6 }], false);
        assert_eq!(16 - 8, kd_tree.count_values());
    }

    #[test]
    fn test_subtract_edge_inside() {
        let mut kd_tree = KdTreeSet::new();
        kd_tree.set_cuboid([Range { min: 0, max: 8 }; 2], true);
        kd_tree.set_cuboid([Range { min: 4, max: 12 }, Range { min: 2, max: 6 }], false);
        assert_eq!(64 - 16, kd_tree.count_values());
    }
}
