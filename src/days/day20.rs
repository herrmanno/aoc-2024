//! # Day 20

use aoc_runner::Day;
use fxhash::FxHashSet;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::common::dir::Dir;

type Num = i16;
type Coord = (Num, Num);

fn distance(a: Coord, b: Coord) -> usize {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as usize
}

#[derive(Default, Clone)]
struct Maze {
    walls: FxHashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl Maze {
    fn get_cheats<const THRESHOLD: usize>(&self, cheat_len: usize) -> usize {
        let course = self.get_course();

        course
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                let mut num_cheats = 0;
                let mut j = i + 1;
                while j < course.len() {
                    let q = course[j];
                    let d = distance(p.0, q.0);
                    // if distance(p, q) > max_cheat_length, advance q to skip tne next
                    // distance(p, q) - max_cheat_length spots that are definitely also
                    // too far away to be a valid cheat target.
                    if d > cheat_len {
                        j += d - cheat_len;
                        continue;
                    }

                    if d <= cheat_len && (q.1 - p.1 - d) >= THRESHOLD {
                        num_cheats += 1;
                    }

                    j += 1;
                }
                num_cheats
            })
            .sum()
    }

    fn get_course(&self) -> Vec<(Coord, usize)> {
        let mut course: Vec<(Coord, usize)> = Default::default();
        let mut pos = self.start;
        let mut step = 0;

        let mut dir = *Dir::ALL
            .iter()
            .find(|d| !self.walls.contains(&d.go(self.start)))
            .expect("No start direction");

        loop {
            course.push((pos, step));

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
