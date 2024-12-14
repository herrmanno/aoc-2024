//! # Day 10

use std::collections::VecDeque;

use aoc_runner::Day;
use fxhash::{FxHashMap, FxHashSet};

type Num = i16;
type Coord = (Num, Num);
type Height = u8;
type Map = FxHashMap<Coord, Height>;

#[derive(Default, Clone)]
pub struct Day10(Map);

impl Day for Day10 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((y as Num, x as Num), ch.to_digit(10).unwrap() as Height))
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0.iter().map(|(pos, h)| self.walk(*pos, *h).0).sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0.iter().map(|(pos, h)| self.walk(*pos, *h).1).sum()
    }
}

impl Day10 {
    fn walk(&self, pos: Coord, height: Height) -> (<Self as Day>::Result1, <Self as Day>::Result2) {
        if height != 0 {
            return (0, 0);
        }

        let mut trail_ends: FxHashSet<Coord> = Default::default();
        let mut num_trails = 0;
        let mut queue = VecDeque::from([(pos, height)]);
        while let Some((pos, height)) = queue.pop_front() {
            if height == 9 {
                trail_ends.insert(pos);
                num_trails += 1;
                continue;
            }

            for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 + dy, pos.1 + dx);
                let new_heigt = self.0.get(&new_pos).cloned().unwrap_or(255);
                if new_heigt == height + 1 {
                    queue.push_back((new_pos, new_heigt));
                }
            }
        }

        (trail_ends.len() as <Self as Day>::Result1, num_trails)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn part_1() {
        let mut day = Day10::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 36);
    }

    #[test]
    fn part_2() {
        let mut day = Day10::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 81);
    }
}
