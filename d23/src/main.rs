use itertools::Itertools;
use std::cmp::Reverse;
use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};

const POW_10: [usize; 4] = [1, 10, 100, 1000];

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Critter {
    A,
    B,
    C,
    D,
}

fn get_room_idx(c: &Critter) -> usize {
    *c as usize
}

fn get_multiplier(c: &Critter) -> usize {
    POW_10[get_room_idx(c)]
}

fn abs_diff(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as usize
}

fn get_room_pos(room_idx: usize) -> usize {
    2 + room_idx * 2
}

type MaybeCritter = Option<Critter>;

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct State<const RS: usize> {
    hallway: [MaybeCritter; 11],
    rooms: [[MaybeCritter; RS]; 4],
}

impl<const RS: usize> State<RS> {
    pub fn from_critters(critters: &Vec<char>) -> Self {
        let mut state = Self {
            hallway: [None; 11],
            rooms: [[None; RS]; 4],
        };

        for (i, c) in critters.iter().enumerate() {
            state.rooms[i % 4][i / 4] = Some(match c {
                'A' => Critter::A,
                'B' => Critter::B,
                'C' => Critter::C,
                'D' => Critter::D,
                _ => unreachable!(),
            })
        }

        state
    }

    pub fn calc_heuristic(&self) -> usize {
        let mut res: usize = 0;
        let mut moved_to_room_cnt = [0; 4];

        for (i, mc) in self.hallway.iter().enumerate() {
            if let Some(c) = mc {
                let room_idx = get_room_idx(c);
                res += abs_diff(i, get_room_pos(room_idx)) * get_multiplier(c);
                moved_to_room_cnt[room_idx] += 1;
            }
        }

        for (room_idx, room) in self.rooms.iter().enumerate() {
            let this_room_pos = get_room_pos(room_idx);

            if self.only_owners_in_room(room_idx) {
                continue;
            }

            // If we have non-owners in the room, assume every critter has to move out and in again,
            // even if this is its own room.
            for (j, mc) in room.iter().enumerate() {
                if let Some(c) = mc {
                    let own_room = get_room_idx(c);
                    let mut addendum = abs_diff(get_room_pos(own_room), this_room_pos); // Travel between rooms
                    addendum += j + 1; // Travel out of this room
                                       // If a critter is moving out of its own room it has to step aside at least once, and then back.
                    if own_room == room_idx {
                        addendum += 2;
                    }

                    addendum *= get_multiplier(c);
                    res += addendum;
                    moved_to_room_cnt[own_room] += 1;
                }
            }
        }

        // Cost of moving into rooms is 1 + 2 + ...
        for (room_idx, moved_cnt) in moved_to_room_cnt.iter().enumerate() {
            let move_in_cost = moved_cnt * (moved_cnt + 1) * POW_10[room_idx] / 2;
            res += move_in_cost
        }

        res
    }

    fn only_owners_in_room(&self, room_idx: usize) -> bool {
        self.rooms[room_idx].iter().all(|mc| match mc {
            None => true,
            Some(c) => get_room_idx(c) == room_idx,
        })
    }

    fn hallway_obstructed(&self, from: usize, to: usize) -> bool {
        !self.hallway[min(from, to)..=max(from, to)]
            .iter()
            .all(|mc| mc.is_none())
    }

    fn gen_moves(&self) -> Vec<(usize, Self)> {
        let mut res = vec![];

        // Try to move critters from hallway into their rooms.
        for (i, mc) in self.hallway.iter().enumerate() {
            if let Some(c) = mc {
                let target_room_idx = get_room_idx(c);

                if !self.only_owners_in_room(target_room_idx) {
                    continue;
                }

                let mut new_state = self.clone();
                new_state.hallway[i] = None;

                if new_state.hallway_obstructed(i, get_room_pos(target_room_idx)) {
                    continue;
                }

                let critters_in_target_room = self.rooms[target_room_idx].iter().filter(|c| c.is_some()).count();
                let travel_dist = abs_diff(i, get_room_pos(target_room_idx));
                let move_in_dist = RS - critters_in_target_room;
                let cost = (travel_dist + move_in_dist) * get_multiplier(c);

                new_state.rooms[target_room_idx][RS - critters_in_target_room - 1] = *mc;
                res.push((cost, new_state));

                // res += abs_diff(i, get_room_pos(room)) * get_multiplier(c);
                // moved_to_room_cnt[room] += 1;
            }
        }

        // Try to move critters out of the rooms into hallway.
        for (room_idx, room) in self.rooms.iter().enumerate() {
            if self.only_owners_in_room(room_idx) {
                continue;
            }

            let room_pos = get_room_pos(room_idx);

            for (pos_in_room, mc) in room.iter().enumerate() {
                if let Some(c) = mc {
                    for (i, _) in self.hallway.iter().enumerate() {
                        if i == 2 || i == 4 || i == 6 || i == 8 {
                            continue;
                        }

                        if self.hallway_obstructed(i, room_pos) {
                            continue;
                        }

                        let travel_dist = abs_diff(room_pos, i);
                        let move_out_dist = pos_in_room + 1;
                        let cost = (travel_dist + move_out_dist) * get_multiplier(c);

                        let mut new_state = self.clone();
                        new_state.hallway[i] = *mc;
                        new_state.rooms[room_idx][pos_in_room] = None;

                        res.push((cost, new_state));
                    }
                    break;
                }
            }
        }

        res
    }

    fn is_final(&self) -> bool {
        self.rooms.iter().enumerate().all(|(room_idx, room)| {
            room.iter()
                .all(|mc| mc.is_some() && get_room_idx(&mc.unwrap()) == room_idx)
        })
    }
}

fn astar_solve<const RS: usize>(critters: &Vec<char>) {
    let s = State::<RS>::from_critters(critters);
    let hh = s.calc_heuristic();

    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(hh), 0, s));

    while !to_visit.is_empty() {
        let (cost_plus_heurisric, cost, state) = to_visit.pop().unwrap();

        let Reverse(cost_plus_heurisric) = cost_plus_heurisric;

        if state.is_final() {
            println!("Total cost: {}", cost_plus_heurisric);
            assert_eq!(state.calc_heuristic(), 0);
            break;
        }

        let moves = state.gen_moves();

        for (move_cost, new_state) in moves {
            let new_cost = cost + move_cost;
            let new_cost_plus_heuristic = new_cost + new_state.calc_heuristic();
            to_visit.push((Reverse(new_cost_plus_heuristic), new_cost, new_state));
        }
    }
}

fn solve(part_1: bool) {
    let reader = BufReader::new(std::io::stdin());

    let mut lines = reader.lines().map(|l| l.unwrap()).collect_vec();
    if !part_1 {
        lines.splice(3..3, [String::from("  #D#C#B#A#"), String::from("  #D#B#A#C#")]);
    }
    let input = lines.join("");
    let critters = input.chars().filter(|c| c >= &'A' && c <= &'D').collect_vec();

    if part_1 {
        astar_solve::<2>(&critters)
    } else {
        astar_solve::<4>(&critters)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_part_1 = args[1] == "1";
    solve(is_part_1);
}
