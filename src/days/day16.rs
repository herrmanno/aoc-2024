//! # Day 16

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use aoc_runner::Day;
use fxhash::{FxHashMap, FxHashSet};

use crate::common::dir::Dir;

type Num = i32;
type Coord = (Num, Num);

#[derive(Debug, Default, Clone)]
struct Maze {
    walls: FxHashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl Maze {
    fn cheapest_path(&mut self) -> Option<usize> {
        type State = Reverse<(usize, Dir, Coord)>;
        let mut visited: FxHashSet<Coord> = Default::default();
        let mut agenda: BinaryHeap<State> = Default::default();
        agenda.push(Reverse((0, Dir::E, self.start)));

        while let Some(Reverse(state)) = agenda.pop() {
            if state.2 == self.end {
                return Some(state.0);
            }
            if self.walls.contains(&state.2) {
                continue;
            }
            if !visited.insert(state.2) {
                continue;
            }

            for d in Dir::ALL {
                let new_state = (
                    state.0 + if d == state.1 { 1 } else { 1001 },
                    d,
                    d.go(state.2),
                );
                agenda.push(Reverse(new_state));
            }
        }

        None
    }

    fn num_best_places(&mut self) -> usize {
        /// The cheapest predecessors of (Coord, Dir) tuple
        struct Predecessor {
            cost: usize,
            coords: FxHashSet<(Coord, Dir)>,
        }

        impl Predecessor {
            fn new() -> Self {
                Self {
                    cost: usize::MAX,
                    coords: Default::default(),
                }
            }

            fn update(&mut self, cost: usize, pos: (Coord, Dir)) {
                if cost < self.cost {
                    self.cost = cost;
                    self.coords.clear();
                }
                if cost <= self.cost {
                    self.coords.insert(pos);
                }
            }
        }

        /// The search state
        #[derive(PartialEq, Eq)]
        struct State {
            cost: usize,
            dir: Dir,
            pos: Coord,
            pred: Option<(Coord, Dir)>,
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.cost.cmp(&other.cost)
            }
        }

        let mut cheapest_path_length = usize::MAX;
        let mut visited: FxHashSet<(Coord, Dir)> = Default::default();
        let mut agenda: BinaryHeap<Reverse<State>> = Default::default();
        agenda.push(Reverse(State {
            cost: 0,
            dir: Dir::E,
            pos: self.start,
            pred: None,
        }));

        let mut predeseccors: FxHashMap<(Coord, Dir), Predecessor> = Default::default();
        predeseccors.insert((self.start, Dir::E), Predecessor::new());

        // (Coordinate, Direction) tuples of path's that are cheapest paths to the target coordinate
        let mut target_pos_dir_tubles: FxHashSet<(Coord, Dir)> = Default::default();

        while let Some(Reverse(state)) = agenda.pop() {
            if let Some(pred) = state.pred {
                predeseccors
                    .entry((state.pos, state.dir))
                    .or_insert(Predecessor::new())
                    .update(state.cost, pred);
            }

            if state.pos == self.end {
                if state.cost > cheapest_path_length {
                    // there are now cheaper paths to explore
                    break;
                } else {
                    cheapest_path_length = state.cost;
                    target_pos_dir_tubles.insert((state.pos, state.dir));
                    continue;
                }
            }

            if !visited.insert((state.pos, state.dir)) {
                continue;
            }

            // new states by turning around
            for d in [state.dir.turn_left(), state.dir.turn_right()] {
                let new_state = State {
                    cost: state.cost + 1000,
                    dir: d,
                    pos: state.pos,
                    pred: Some((state.pos, state.dir)),
                };

                if !visited.contains(&(new_state.pos, new_state.dir)) {
                    agenda.push(Reverse(new_state));
                }
            }

            // new states by going forward
            let new_state = State {
                cost: state.cost + 1,
                dir: state.dir,
                pos: state.dir.go(state.pos),
                pred: Some((state.pos, state.dir)),
            };

            if !self.walls.contains(&new_state.pos)
                && !visited.contains(&(new_state.pos, new_state.dir))
            {
                agenda.push(Reverse(new_state));
            }
        }

        // Go found paths backwards by traversing the predecessors map
        let mut best_seats: FxHashSet<Coord> = Default::default();
        let mut q: VecDeque<(Coord, Dir)> = target_pos_dir_tubles.into_iter().collect();
        while let Some(c) = q.pop_front() {
            for c in predeseccors
                .remove(&c)
                .map(|it| it.coords)
                .unwrap_or_default()
            {
                q.push_back(c);
            }
            best_seats.insert(c.0);
        }

        best_seats.len()
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut walls: FxHashSet<Coord> = Default::default();
        let mut start: Coord = Default::default();
        let mut end: Coord = Default::default();

        for (y, line) in value.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        walls.insert((y as Num, x as Num));
                    }
                    'S' => {
                        start = (y as Num, x as Num);
                    }
                    'E' => {
                        end = (y as Num, x as Num);
                    }
                    _ => {}
                }
            }
        }

        Self { walls, start, end }
    }
}

#[derive(Default, Clone)]
pub struct Day16(Maze);

impl Day for Day16 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input.into();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0.cheapest_path().expect("No solution") as <Self as Day>::Result1
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0.num_best_places() as <Self as Day>::Result2
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT1: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    const INPUT2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    fn part_1() {
        let mut day = Day16::default();
        day.parse(INPUT1);
        assert_eq!(day.part1(), 7036);

        let mut day = Day16::default();
        day.parse(INPUT2);
        assert_eq!(day.part1(), 11048);
    }

    #[test]
    fn part_2() {
        let mut day = Day16::default();
        day.parse(INPUT1);
        assert_eq!(day.part2(), 45);

        let mut day = Day16::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 64);
    }
}
