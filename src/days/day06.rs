//! # Day 06

use aoc_runner::Day;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::common::dir::Dir;

type Coord = (i16, i16);

#[derive(Default, Clone)]
pub struct Day06 {
    start_pos: Coord,
    obstacles: HashSet<Coord>,
    size: [i16; 2],
}

impl Day for Day06 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.size = {
            let y = input.lines().count();
            let x = input.lines().next().unwrap().chars().count();
            [y as i16, x as i16]
        };

        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((y as i16, x as i16), ch))
            })
            .for_each(|(pos, ch)| match ch {
                '#' => {
                    self.obstacles.insert(pos);
                }
                '^' => {
                    self.start_pos = pos;
                }
                _ => {}
            });
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut visited: HashSet<Coord> = Default::default();
        let mut direction = Dir::N;
        let mut pos = self.start_pos;
        loop {
            visited.insert(pos);
            let next_pos = direction.go(pos);

            if self.outside_map(&next_pos) {
                break;
            }

            if self.obstacles.contains(&next_pos) {
                direction = direction.turn_right();
            } else {
                pos = next_pos;
            }
        }
        visited.len() as <Self as Day>::Result1
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut direction = Dir::N;
        let mut visited: HashSet<Coord> = Default::default();
        let mut pos = self.start_pos;
        loop {
            visited.insert(pos);
            let next_pos = direction.go(pos);

            if self.outside_map(&next_pos) {
                break;
            }

            if self.obstacles.contains(&next_pos) {
                direction = direction.turn_right();
            } else {
                pos = next_pos;
            }
        }

        visited
            .par_iter()
            .filter(|pos| self.walk(self.start_pos, Dir::N, **pos))
            .count() as <Self as Day>::Result2
    }
}

impl Day06 {
    fn walk(&self, start: Coord, mut direction: Dir, obstacle: Coord) -> bool {
        let mut visited: HashMap<Coord, HashSet<Dir>> = Default::default();
        let mut pos = start;

        loop {
            let next_pos = direction.go(pos);

            if self.outside_map(&next_pos) {
                return false;
            }

            if !visited.entry(pos).or_default().insert(direction) {
                return true;
            }

            if self.obstacles.contains(&next_pos) || next_pos == obstacle {
                direction = direction.turn_right();
            } else {
                pos = next_pos;
            }
        }
    }

    #[inline]
    fn outside_map(&self, pos: &Coord) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size[0] || pos.1 >= self.size[1]
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part_1() {
        let mut day = Day06::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 41);
    }

    #[test]
    fn part_2() {
        let mut day = Day06::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 6);
    }
}
