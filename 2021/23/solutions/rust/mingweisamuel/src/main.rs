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

fn pathfind_dist<const N: usize, const PATH: bool>(
    start: BurrowState<N>,
) -> Option<Vec<(u32, BurrowState<N>)>> {
    let mut queue = PriorityQueue::new();
    let mut dist: HashMap<BurrowState<N>, u32> = Default::default();
    let mut prev: HashMap<BurrowState<N>, BurrowState<N>> = Default::default();
    queue.push(start, Reverse(0));
    dist.insert(start, 0);

    while let Some((burrow, Reverse(_est_total_dist))) = queue.pop() {
        let d = dist[&burrow];

        if burrow.is_goal() {
            let mut burrow = burrow;

            let mut path = vec![(d, burrow)];
            if PATH {
                while burrow != start {
                    burrow = *prev.get(&burrow).unwrap();
                    path.push((dist[&burrow], burrow));
                }
                path.reverse();
            }
            return Some(path);
        }

        burrow.visit_neighbors(|edge_cost, next_burrow| {
            let next_dist = d + edge_cost;
            if dist
                .get(&next_burrow)
                .map(|&old_dist| next_dist < old_dist)
                .unwrap_or(true)
            {
                dist.insert(next_burrow, next_dist);
                if PATH {
                    prev.insert(next_burrow, burrow);
                }
                let heuristic = next_burrow.heuristic();
                queue.push(next_burrow, Reverse(next_dist + heuristic));
            }
        });
    }
    None
}

fn main() {
    const N: usize = 16;
    // // example.
    let state = BurrowState::<N>::from_initial([
        [Amphipod::B, Amphipod::C, Amphipod::B, Amphipod::D],
        [Amphipod::D, Amphipod::C, Amphipod::B, Amphipod::A],
        [Amphipod::D, Amphipod::B, Amphipod::A, Amphipod::C],
        [Amphipod::A, Amphipod::D, Amphipod::C, Amphipod::A],
    ]);

    // mingweisamuel
    let state = BurrowState::<N>::from_initial([
        [Amphipod::C, Amphipod::A, Amphipod::B, Amphipod::D],
        [Amphipod::D, Amphipod::C, Amphipod::B, Amphipod::A],
        [Amphipod::D, Amphipod::B, Amphipod::A, Amphipod::C],
        [Amphipod::D, Amphipod::C, Amphipod::A, Amphipod::B],
    ]);
    // 43450: too low.

    // println!("{}", state);

    // state.visit_neighbors(|cost, next_burrow| {
    //     println!("{}\n{}", cost, next_burrow);
    //     let pos = Position {
    //         room: None,
    //         index: 3,
    //     };
    //     if Some(Amphipod::B) == next_burrow.get_position(pos) {
    //         next_burrow.visit_neighbors(|cost2, next_burrow| {
    //             println!("{}->{}\n{}", cost, cost2, next_burrow);
    //             let pos_a = Position {
    //                 room: None,
    //                 index: 5,
    //             };
    //             let pos_b = Position {
    //                 room: Some(Amphipod::B),
    //                 index: 0,
    //             };
    //             if Some(Amphipod::C) == next_burrow.get_position(pos_a)
    //                 && next_burrow.get_position(pos_b).is_none()
    //             {
    //                 next_burrow.visit_neighbors(|cost3, next_burrow| {
    //                     println!("{}->{}->{}\n{}", cost, cost2, cost3, next_burrow);
    //                 });
    //             }
    //         });
    //     }
    // });

    let path = pathfind_dist::<N, false>(state).unwrap();
    for (cost, state) in path {
        println!("{}\n{}", cost, state);
    }
}

// fn main() {
//     // let stdin = std::io::stdin();
//     // let grid: Vec<Vec<u8>> = stdin
//     //     .lock()
//     //     .lines()
//     //     .map(|line| line.expect("Failed to read line as UTF-8."))
//     //     .map(|line| line.into_bytes())
//     //     .map(|mut row| {
//     //         for b in row.iter_mut() {
//     //             *b -= b'0';
//     //         }
//     //         row
//     //     })
//     //     .collect();

//     // let size = grid.len();
//     // assert_eq!(size, grid[0].len());

//     // let bound = 5 * size - 1;

//     // A: 1
//     // B: 2
//     // C: 3
//     // D: 4

//     // Example.
//     let start = AmphipodBurrow {
//         hallway: Default::default(),
//         rooms: [[1, 2], [4, 3], [3, 2], [1, 4]],
//     };

//     // mingweisamuel
//     let start = AmphipodBurrow {
//         hallway: Default::default(),
//         rooms: [[4, 3], [3, 1], [1, 2], [2, 4]],
//     };

//     start.next_steps(|cost, next_burrow| {
//         println!("{}\n{}", cost, next_burrow);
//         if next_burrow.hallway[3] == 2 {
//             next_burrow.next_steps(|cost2, next_burrow| {
//                 println!("{}->{}\n{}", cost, cost2, next_burrow);
//                 if next_burrow.hallway[5] == 3 && next_burrow.rooms[1][1] == 0 {
//                     next_burrow.next_steps(|cost3, next_burrow| {
//                         println!("{}->{}->{}\n{}", cost, cost2, cost3, next_burrow);
//                     });
//                 }
//             });
//         }
//     });

//     let part_a = pathfind_dist(start).expect("No path found.");
//     // let part_a = pathfind_dist(&*grid, size - 1);
//     // let part_b = pathfind_dist(&*grid, bound);
//     let part_b = part_a;

//     println!("{}\n{}", part_a, part_b);
// }

// const COSTS: [usize; 5] = [10_000_000, 1, 10, 100, 1000];

// const fn format_state(state: u8) -> char {
//     match state {
//         0 => '.',
//         1 => 'A',
//         2 => 'B',
//         3 => 'C',
//         4 => 'D',
//         _ => panic!(),
//     }
// }
// const fn room_for_amphipod(state: u8) -> usize {
//     (state - 1) as usize
// }
// const fn room_hallway_pos(room: usize) -> usize {
//     2 * (room + 1)
// }
// const fn hallway_doorway(hallway_pos: usize) -> bool {
//     match hallway_pos {
//         2 | 4 | 6 | 8 => true,
//         _ => false,
//     }
// }

// ///
// /// ```txt
// /// #############
// /// #01234567890#
// /// ###1#1#1#1###
// ///   #0#0#0#0#
// ///   #########
// /// ```
// ///
// #[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
// pub struct AmphipodBurrow {
//     hallway: [u8; 11],
//     rooms: [[u8; 2]; 4],
// }
// impl AmphipodBurrow {
//     pub fn is_goal(&self) -> bool {
//         0 == self.heuristic()
//     }
//     pub fn heuristic(&self) -> usize {
//         let mut cost = 0;
//         // Amphipods in hallway.
//         for (start_pos, state) in self.hallway.into_iter().enumerate() {
//             if 0 != state {
//                 let move_cost = COSTS[state as usize];
//                 let room_index = room_for_amphipod(state);
//                 cost += move_cost * (max(room_index, start_pos) - min(room_index, start_pos));
//                 cost += move_cost;
//             }
//         }
//         // Amphipods in rooms.
//         for room_index in 0..4 {
//             for room_back_front in 0..=1 {
//                 let state = self.rooms[room_index][room_back_front];
//                 if 0 != state {
//                     let room_target_index = room_for_amphipod(state);
//                     if room_target_index != room_index {
//                         let move_cost = COSTS[state as usize];
//                         cost += move_cost
//                             * (max(room_index, room_target_index)
//                                 - min(room_index, room_target_index));
//                         cost += move_cost * 2;
//                         cost += move_cost * (1 - room_back_front);
//                     }
//                 }
//             }
//         }
//         cost
//     }
//     pub fn next_steps(&self, mut visit: impl FnMut(usize, AmphipodBurrow)) {
//         // Move from hallway to room.
//         'outer: for (start_pos, state) in self.hallway.into_iter().enumerate() {
//             if 0 != state {
//                 // println!("X");
//                 let move_cost = COSTS[state as usize];
//                 let room_index = room_for_amphipod(state);
//                 let room_pos = room_hallway_pos(room_index);

//                 if 0 != self.rooms[room_index][1] {
//                     // println!("A");
//                     continue 'outer;
//                 }

//                 let mut cost = 0;
//                 for pos in min(start_pos + 1, room_pos)..=max(start_pos.saturating_sub(1), room_pos)
//                 {
//                     if 0 != self.hallway[pos] {
//                         // println!(
//                         //     "B {}->{} blocked by {}",
//                         //     min(start_pos, room_pos),
//                         //     max(start_pos, room_pos),
//                         //     pos
//                         // );
//                         continue 'outer;
//                     }
//                     cost += move_cost;
//                 }
//                 // Move into back of room.
//                 if 0 == self.rooms[room_index][0] {
//                     let mut next = self.clone();
//                     next.hallway[start_pos] = 0;
//                     next.rooms[room_index][0] = state;
//                     (visit)(cost + 2 * move_cost, next);
//                 }
//                 // Move into front of room.
//                 else if state == self.rooms[room_index][0] {
//                     let mut next = self.clone();
//                     next.hallway[start_pos] = 0;
//                     next.rooms[room_index][1] = state;
//                     (visit)(cost + move_cost, next);
//                 }
//             }
//         }
//         // Move from room to hallway.
//         for room_index in 0..4 {
//             let room_hw_pos = room_hallway_pos(room_index);

//             let (state, room_front_back, start_steps) = if 0 != self.rooms[room_index][1] {
//                 // Move out of front of room.
//                 (self.rooms[room_index][1], 1, 1)
//             } else if 0 != self.rooms[room_index][0] {
//                 // Move out of back of room.
//                 (self.rooms[room_index][0], 0, 2)
//             } else {
//                 continue;
//             };

//             let target_room_index = room_for_amphipod(state);
//             if room_index == target_room_index {
//                 #[allow(clippy::if_same_then_else)]
//                 if room_front_back == 0 {
//                     // Back already in right spot.
//                     continue;
//                 } else if self.rooms[room_index][0] == state {
//                     // Both already in right spot.
//                     continue;
//                 }
//             }

//             let move_cost = COSTS[state as usize];
//             let start_cost = move_cost * start_steps;
//             // Go left.
//             {
//                 let mut cum_cost = start_cost;
//                 for hallway_pos in (0..=room_hw_pos).rev() {
//                     if 0 != self.hallway[hallway_pos] {
//                         break;
//                     }
//                     if !hallway_doorway(hallway_pos) {
//                         let mut next = self.clone();
//                         next.hallway[hallway_pos] = state;
//                         next.rooms[room_index][room_front_back] = 0;
//                         (visit)(cum_cost, next);
//                     }
//                     cum_cost += move_cost;
//                 }
//             }
//             // Go right.
//             {
//                 let mut cum_cost = start_cost;
//                 for hallway_pos in room_hw_pos..=10 {
//                     if 0 != self.hallway[hallway_pos] {
//                         break;
//                     }
//                     if !hallway_doorway(hallway_pos) {
//                         let mut next = self.clone();
//                         next.hallway[hallway_pos] = state;
//                         next.rooms[room_index][room_front_back] = 0;
//                         (visit)(cum_cost, next);
//                     }
//                     cum_cost += move_cost;
//                 }
//             }
//         }
//     }
// }
// impl std::fmt::Display for AmphipodBurrow {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         writeln!(f, "#############")?;
//         writeln!(
//             f,
//             "#{}{}{}{}{}{}{}{}{}{}{}#",
//             format_state(self.hallway[0]),
//             format_state(self.hallway[1]),
//             format_state(self.hallway[2]),
//             format_state(self.hallway[3]),
//             format_state(self.hallway[4]),
//             format_state(self.hallway[5]),
//             format_state(self.hallway[6]),
//             format_state(self.hallway[7]),
//             format_state(self.hallway[8]),
//             format_state(self.hallway[9]),
//             format_state(self.hallway[10]),
//         )?;
//         writeln!(
//             f,
//             "###{}#{}#{}#{}###",
//             format_state(self.rooms[0][1]),
//             format_state(self.rooms[1][1]),
//             format_state(self.rooms[2][1]),
//             format_state(self.rooms[3][1])
//         )?;
//         writeln!(
//             f,
//             "###{}#{}#{}#{}###",
//             format_state(self.rooms[0][0]),
//             format_state(self.rooms[1][0]),
//             format_state(self.rooms[2][0]),
//             format_state(self.rooms[3][0])
//         )?;
//         writeln!(f, "  #########")?;
//         Ok(())
//     }
// }
