//! # Day 18

use std::collections::VecDeque;

use aoc_runner::Day;
use bit_set::BitSet;

use crate::common::dir::Dir;

type Num = i16;
type Coord = (Num, Num);
type Blocks = BitSet<usize>;

#[derive(Default, Clone)]
pub struct Day18<const SIZE: usize = 70, const NUM_BLOCKS: usize = 1024>(Vec<Coord>);

impl<const SIZE: usize, const NUM_BLOCKS: usize> Day18<SIZE, NUM_BLOCKS> {
    fn coord_to_index(coord: Coord) -> usize {
        coord.0 as usize * (SIZE + 1) + coord.1 as usize
    }

    fn is_valid(&self, coord: Coord, blocks: &Blocks) -> bool {
        coord.0 >= 0
            && coord.0 <= SIZE as Num
            && coord.1 >= 0
            && coord.1 <= SIZE as Num
            && !blocks.contains(Self::coord_to_index(coord))
    }

    fn search(&self, blocks: &Blocks) -> Option<u32> {
        let start = (0, 0);
        let end = (SIZE as Num, SIZE as Num);

        let mut agenda: VecDeque<(Coord, u32)> = VecDeque::from([(start, 0)]);
        let mut visited: BitSet<usize> = Default::default();
        while let Some((coord, cost)) = agenda.pop_front() {
            if !visited.insert(Self::coord_to_index(coord)) {
                continue;
            }
            if coord == end {
                return Some(cost);
            }
            for d in Dir::ALL {
                let coord = d.go(coord);
                let cost = cost + 1;
                if self.is_valid(coord, blocks) {
                    agenda.push_back((coord, cost));
                }
            }
        }

        None
    }
}

impl<const SIZE: usize, const NUM_BLOCKS: usize> Day for Day18<SIZE, NUM_BLOCKS> {
    type Result1 = u32;
    type Result2 = String;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                (y.parse().unwrap(), x.parse().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let blocks = self
            .0
            .iter()
            .cloned()
            .take(NUM_BLOCKS)
            .map(Self::coord_to_index)
            .collect();
        self.search(&blocks).expect("No solution found")
    }

    fn part2(&mut self) -> Self::Result2 {
        let indices = (NUM_BLOCKS..=self.0.len()).collect::<Vec<_>>();
        let index = indices.as_slice().partition_point(|&i| {
            let blocks = self
                .0
                .iter()
                .cloned()
                .take(i)
                .map(Self::coord_to_index)
                .collect();
            self.search(&blocks).is_some()
        });
        let index = index + (NUM_BLOCKS - 1);

        format!("{},{}", self.0[index].1, self.0[index].0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn part_1() {
        let mut day: Day18<6, 12> = Day18::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 22);
    }

    #[test]
    fn part_2() {
        let mut day: Day18<6, 12> = Day18::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), "6,1");
    }
}
