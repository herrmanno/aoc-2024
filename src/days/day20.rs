//! # Day 20

use std::collections::VecDeque;

use aoc_runner::Day;
use bit_set::BitSet;
use fxhash::{FxHashMap, FxHashSet};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::common::dir::Dir;

type Num = i16;
type Coord = (Num, Num);

#[derive(Default, Clone)]
struct Maze {
    walls: FxHashSet<Coord>,
    start: Coord,
    end: Coord,
    size: (Num, Num),
}

impl Maze {
    fn contains(&self, pos: Coord) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.size.0 && pos.1 < self.size.1
    }

    fn coord_to_usize(&self, (y, x): Coord) -> usize {
        (y * self.size.0 + x) as usize
    }

    fn get_cheats<const THRESHOLD: usize>(&self, cheat_len: usize) -> usize {
        let course = self.get_course();

        course
            .par_iter()
            .map(|(pos, cost)| {
                let mut cheats = 0;
                type State = (Coord, usize);
                let mut visited: BitSet = Default::default();
                let mut agenda: VecDeque<State> = VecDeque::from([(*pos, 0)]);
                while let Some((cheat_pos, len)) = agenda.pop_front() {
                    if len > cheat_len {
                        break;
                    }

                    if let Some(cheat_cost) = course.get(&cheat_pos) {
                        if *cheat_cost > *cost + 2 && (cheat_cost - cost - len) >= THRESHOLD {
                            cheats += 1;
                        }
                    }

                    for d in Dir::ALL {
                        let new_state = (d.go(cheat_pos), len + 1);
                        if self.contains(new_state.0)
                            && visited.insert(self.coord_to_usize(new_state.0))
                        {
                            agenda.push_back(new_state);
                        }
                    }
                }
                cheats
            })
            .reduce(|| 0, |acc, cheats| acc + cheats)
    }

    fn get_course(&self) -> FxHashMap<Coord, usize> {
        let mut course: FxHashMap<Coord, usize> = Default::default();
        let mut pos = self.start;
        let mut step = 0;

        let mut dir = *Dir::ALL
            .iter()
            .find(|d| !self.walls.contains(&d.go(self.start)))
            .expect("No start direction");

        loop {
            course.insert(pos, step);

            if pos == self.end {
                break;
            }

            dir = *[dir, dir.turn_left(), dir.turn_right()]
                .iter()
                .find(|d| !self.walls.contains(&d.go(pos)))
                .expect("Nowhere to go");

            pos = dir.go(pos);

            step += 1;
        }

        course
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let size = (
            value.lines().count() as Num,
            value.lines().next().unwrap().chars().count() as Num,
        );
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
        Self {
            size,
            walls,
            start,
            end,
        }
    }
}

#[derive(Default, Clone)]
pub struct Day20<const THRESHOLD: usize = 100>(Maze);

impl<const THRESHOLD: usize> Day for Day20<THRESHOLD> {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input.into();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0.get_cheats::<THRESHOLD>(2)
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0.get_cheats::<THRESHOLD>(20)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn part_1() {
        let mut day: Day20<2> = Day20::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 44);
    }

    #[test]
    fn part_2() {
        let mut day: Day20<50> = Day20::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 285);
    }
}
