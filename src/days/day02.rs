//! # Day 01 - Historian Hysteria

use std::{borrow::Borrow, cmp::Ordering};

use itertools::Itertools;

use crate::common::{iter::SkipNth, transform::Transform};
use aoc_runner::Day;

type Level = u32;
type Report = Vec<Level>;

struct Safety<T>(T);

impl<L: Clone + Borrow<Level>, T: Iterator<Item = L>> Safety<T> {
    fn new(report: T) -> Self {
        Self(report)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_safe(self) -> bool {
        fn safe(pair: ((u32, Ordering), (u32, Ordering))) -> bool {
            let (a, b) = pair;
            (a.0 >= 1 && a.0 <= 3) && (b.0 >= 1 && b.0 <= 3) && a.1 == b.1
        }

        self.0
            .tuple_windows()
            .map(|(a, b)| (a.borrow().abs_diff(*b.borrow()), b.borrow().cmp(a.borrow())))
            .tuple_windows()
            .all(safe)
    }
}

#[derive(Default, Clone)]
pub struct Day02 {
    reports: Vec<Report>,
}

impl Day for Day02 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.reports = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|part| part.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1(&mut self) -> Self::Result1 {
        self.reports
            .iter()
            .filter(|r| Safety::new(r.iter()).is_safe())
            .count() as Self::Result1
    }

    fn part2(&mut self) -> Self::Result2 {
        self.reports
            .iter()
            .filter(|r| {
                (0..r.len()).any(|i| {
                    r.iter()
                        .skip_nth(i)
                        .transform(|it| Safety::new(it).is_safe())
                })
            })
            .count() as Self::Result1
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part_1() {
        let mut day = Day02::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 2);
    }

    #[test]
    fn part_2() {
        let mut day = Day02::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 4);
    }
}
