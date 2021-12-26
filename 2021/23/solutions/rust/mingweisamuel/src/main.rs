use priority_queue::PriorityQueue;
use std::cmp::{max, min, Reverse};
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
impl Amphipod {
    const ARRAY: [Self; 4] = [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D];
    pub fn from_char_byte(c: u8) -> Self {
        match c {
            b'A' => Self::A,
            b'B' => Self::B,
            b'C' => Self::C,
            b'D' => Self::D,
            c => panic!("Invalid char byte: {}", c),
        }
    }
    pub fn move_cost(self) -> u32 {
        10_u32.pow(self as u32)
    }
    pub fn hallway_doorway(self) -> u8 {
        2 + 2 * (self as u8)
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Position {
    pub room: Option<Amphipod>,
    pub index: u8,
}
impl Position {
    const HALLWAY_LEN: u8 = 11;
    pub fn is_doorway(self) -> bool {
        matches!(
            self,
            Position {
                room: None,
                index: 2 | 4 | 6 | 8,
            }
        )
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BurrowState<const N: usize> {
    positions: [(Position, Amphipod); N],
}
impl<const N: usize> BurrowState<N> {
    const ROOM_DEPTH: u8 = (N / 4) as u8;

    pub fn from_initial(initial: impl IntoIterator<Item = [Amphipod; 4]>) -> Self {
        let mut room_depth = 0;
        let mut positions = [None; N];

        for (i, position_amphipod) in initial
            .into_iter()
            .flat_map(|amphipod_row| {
                let mut i = 0;
                let out = amphipod_row.map(|amphipod| {
                    let out = (
                        Position {
                            room: Some(Amphipod::ARRAY[i as usize]),
                            index: room_depth as u8,
                        },
                        amphipod,
                    );
                    i += 1;
                    out
                });
                room_depth += 1;
                out
            })
            .enumerate()
        {
            *positions.get_mut(i).expect("Too many amphipods") = Some(position_amphipod);
        }

        assert_eq!(Self::ROOM_DEPTH, room_depth);

        let mut positions = positions.map(|opt| opt.unwrap());
        positions.sort_unstable();
        Self { positions }
    }

    pub fn move_new(&self, from: Position, to: Position) -> Self {
        let mut out = *self;
        let (i, amphipod) = self
            .get_position_indexed(from)
            .expect("Cannot move from empty.");
        out.positions[i] = (to, amphipod);
        out.positions.sort_unstable();
        out
    }

    fn get_position_indexed(&self, position: Position) -> Option<(usize, Amphipod)> {
        for (i, &(other_position, amphipod)) in self.positions.iter().enumerate() {
            if other_position > position {
                // Sort optimization.
                return None;
            }
            if other_position == position {
                return Some((i, amphipod));
            }
        }
        None
    }

    pub fn get_position(&self, position: Position) -> Option<Amphipod> {
        self.get_position_indexed(position)
            .map(|(_, amphipod)| amphipod)
    }

    pub fn is_goal(&self) -> bool {
        0 == self.heuristic()
    }

    pub fn heuristic(&self) -> u32 {
        let mut cost = 0;
        for &(position, amphipod) in self.positions.iter() {
            match position {
                Position {
                    room: None,
                    index: hallway_pos,
                } => {
                    // In hallway.
                    let hallway_doorway = amphipod.hallway_doorway();
                    // Move to doorway + move into entrance.
                    let moves =
                        1 + max(hallway_pos, hallway_doorway) - min(hallway_pos, hallway_doorway);
                    cost += (moves as u32) * amphipod.move_cost();
                }
                Position {
                    room: Some(current_room),
                    index: room_depth,
                } => {
                    // Check if already in desination room and not blocking others.
                    if amphipod == current_room {
                        // Aready in destinantion, blocking none.
                        if (room_depth..Self::ROOM_DEPTH).all(|room_depth| {
                            Some(amphipod)
                                == self.get_position(Position {
                                    room: Some(current_room),
                                    index: room_depth,
                                })
                        }) {
                            // No cost.
                            continue;
                        } else {
                            // Cost is to move out then back into room.
                            // Four steps to:
                            // (1) get in hallway_doorway,
                            // (2) get out of doorway,
                            // (3) move back into doorway,
                            // (4) move back into room.
                            let moves = room_depth + 4;
                            cost += (moves as u32) * amphipod.move_cost();
                        }
                    } else {
                        let hallway_start = current_room.hallway_doorway();
                        let hallway_end = amphipod.hallway_doorway();
                        let mut moves = 0;
                        // Move to front of room, then into hallway.
                        moves += room_depth + 1;
                        // Moves in hallway.
                        moves += max(hallway_start, hallway_end) - min(hallway_start, hallway_end);
                        // Move into destination room.
                        moves += 1;
                        cost += (moves as u32) * amphipod.move_cost();
                    }
                }
            }
        }
        cost
    }

    pub fn visit_neighbors(&self, mut visit: impl FnMut(u32, Self)) {
        'outer: for &(position, amphipod) in self.positions.iter() {
            match position {
                Position {
                    room: None,
                    index: hallway_start,
                } => {
                    // Not in room (in hallway), try moving into room.

                    // Find spot in room.
                    if let Some(room_dest) = (0..Self::ROOM_DEPTH)
                        .rev()
                        .map(|room_depth| Position {
                            room: Some(amphipod),
                            index: room_depth,
                        })
                        .take_while(|&pos| {
                            Some(amphipod) == self.get_position(pos)
                                || self.get_position(pos).is_none()
                        })
                        .find(|&pos| self.get_position(pos).is_none())
                    {
                        let mut cost = amphipod.move_cost();

                        // Move backwards, from room to hallway.
                        for room_depth in (0..room_dest.index).rev() {
                            cost += amphipod.move_cost();
                            if self
                                .get_position(Position {
                                    room: Some(amphipod),
                                    index: room_depth,
                                })
                                .is_some()
                            {
                                continue 'outer;
                            }
                        }

                        // Move through hallway (forward).
                        for hallway_index in min(hallway_start + 1, amphipod.hallway_doorway())
                            ..=max(hallway_start.saturating_sub(1), amphipod.hallway_doorway())
                        {
                            cost += amphipod.move_cost();
                            if self
                                .get_position(Position {
                                    room: None,
                                    index: hallway_index,
                                })
                                .is_some()
                            {
                                // Blocked by other amphipod.
                                continue 'outer;
                            }
                        }

                        (visit)(cost, self.move_new(position, room_dest));
                    }
                }
                Position {
                    room: Some(current_room),
                    index: room_start,
                } => {
                    // In room, try moving out into hallway.
                    let mut cost = 0;

                    // Check if already in desination room and not blocking others.
                    if amphipod == current_room
                        && (room_start..Self::ROOM_DEPTH).all(|room_depth| {
                            Some(amphipod)
                                == self.get_position(Position {
                                    room: Some(current_room),
                                    index: room_depth,
                                })
                        })
                    {
                        // Aready in destinantion, blocking none.
                        continue 'outer;
                    }

                    // Move out of room.
                    for room_depth in (0..room_start).rev() {
                        cost += amphipod.move_cost();
                        if self
                            .get_position(Position {
                                room: Some(current_room),
                                index: room_depth,
                            })
                            .is_some()
                        {
                            // Blocked in hallway.
                            continue 'outer;
                        }
                    }
                    let hallway_start = current_room.hallway_doorway();

                    // Move left
                    {
                        let mut cost = cost;
                        for hallway_index in (0..=hallway_start).rev() {
                            cost += amphipod.move_cost();
                            let hallway_pos = Position {
                                room: None,
                                index: hallway_index,
                            };
                            if self.get_position(hallway_pos).is_some() {
                                // Blocked by other amphipod.
                                break;
                            }

                            if !hallway_pos.is_doorway() {
                                (visit)(cost, self.move_new(position, hallway_pos));
                            }
                        }
                    }

                    // Move right
                    {
                        let mut cost = cost;
                        for hallway_index in hallway_start..Position::HALLWAY_LEN {
                            cost += amphipod.move_cost();
                            let hallway_pos = Position {
                                room: None,
                                index: hallway_index,
                            };
                            if self.get_position(hallway_pos).is_some() {
                                // Blocked by other amphipod.
                                break;
                            }

                            if !hallway_pos.is_doorway() {
                                (visit)(cost, self.move_new(position, hallway_pos));
                            }
                        }
                    }
                }
            }
        }
    }
}
impl<const N: usize> std::fmt::Display for BurrowState<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #A#D#C#A#
        //   #########
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for index in 0..Position::HALLWAY_LEN {
            if let Some(amphipod) = self.get_position(Position { room: None, index }) {
                write!(f, "{:?}", amphipod)?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f, "#")?;
        for room_depth in 0..Self::ROOM_DEPTH {
            if 0 == room_depth {
                write!(f, "##")?;
            } else {
                write!(f, "  ")?;
            }
            {
                for room in Amphipod::ARRAY {
                    if let Some(amphipod) = self.get_position(Position {
                        room: Some(room),
                        index: room_depth,
                    }) {
                        write!(f, "#{:?}", amphipod)?;
                    } else {
                        write!(f, "#.")?;
                    }
                }
                write!(f, "#")?;
            }
            if 0 == room_depth {
                write!(f, "##")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  #########")?;
        Ok(())
    }
}

fn pathfind_dist<const N: usize>(start: BurrowState<N>) -> Option<u32> {
    let mut queue = PriorityQueue::new();
    let mut dist: HashMap<BurrowState<N>, u32> = Default::default();
    queue.push(start, Reverse(0));
    dist.insert(start, 0);

    while let Some((burrow, Reverse(dist_est))) = queue.pop() {
        if burrow.is_goal() {
            return Some(dist_est);
        }

        let d = dist[&burrow];
        burrow.visit_neighbors(|edge_cost, next_burrow| {
            let next_dist = d + edge_cost;
            if dist
                .get(&next_burrow)
                .map(|&old_dist| next_dist < old_dist)
                .unwrap_or(true)
            {
                dist.insert(next_burrow, next_dist);
                let heuristic = next_burrow.heuristic();
                queue.push(next_burrow, Reverse(next_dist + heuristic));
            }
        });
    }
    None
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));

    assert_eq!(Some("#############"), lines.next().as_deref());
    assert_eq!(Some("#...........#"), lines.next().as_deref());
    let row_0 = {
        let line = lines.next().unwrap();
        let line = line.as_bytes();
        [line[3], line[5], line[7], line[9]].map(Amphipod::from_char_byte)
    };
    let row_n = {
        let line = lines.next().unwrap();
        let line = line.as_bytes();
        [line[3], line[5], line[7], line[9]].map(Amphipod::from_char_byte)
    };
    assert_eq!(Some("  #########"), lines.next().as_deref());

    let part_a = std::thread::spawn(move || {
        let state = BurrowState::<8>::from_initial([row_0, row_n]);
        pathfind_dist(state)
    });

    let part_b = {
        let state = BurrowState::<16>::from_initial([
            row_0,
            [Amphipod::D, Amphipod::C, Amphipod::B, Amphipod::A],
            [Amphipod::D, Amphipod::B, Amphipod::A, Amphipod::C],
            row_n,
        ]);
        pathfind_dist(state)
    };

    let part_a = part_a.join().unwrap();
    println!(
        "{}\n{}",
        part_a.expect("Failed to solve part A."),
        part_b.expect("Failed to solve part B.")
    );
}
