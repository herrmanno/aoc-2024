//! # Day 01 - Historian Hysteria

use fxhash::FxHashMap as HashMap;

use crate::common::transform::Transform;
use aoc_runner::Day;

type LocationId = u32;
type LocationList = Vec<LocationId>;

#[derive(Default, Clone)]
pub struct Day01 {
    lists: [LocationList; 2],
}

impl Day for Day01 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.lists = input
            .lines()
            .map(|line| line.split_whitespace())
            .map(|parts| {
                let mut numbers = parts.map(|part| part.parse::<LocationId>().unwrap());
                (numbers.next().unwrap(), numbers.next().unwrap())
            })
            .unzip()
            .transform(|(a, b)| [a, b]);
    }

    fn part1(&mut self) -> Self::Result1 {
        self.lists.iter_mut().for_each(|list| list.sort_unstable());

        self.lists[0]
            .iter()
            .zip(self.lists[1].iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        let counts: HashMap<LocationId, u32> =
            self.lists[1].iter().fold(Default::default(), |mut acc, n| {
                acc.entry(*n).and_modify(|count| *count += 1).or_insert(1);
                acc
            });

        self.lists[0]
            .iter()
            .map(|n| n * counts.get(n).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn part_1() {
        let mut day = Day01::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 11);
    }

    #[test]
    fn part_2() {
        let mut day = Day01::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 31);
    }
}
