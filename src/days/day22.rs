//! # Day 22

use aoc_runner::Day;
use fxhash::FxHashMap;
use itertools::iterate;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Num = u64;

fn next_secret(mut i: Num) -> Num {
    i = (i ^ (i << 6)) % 16777216;
    i = (i ^ (i >> 5)) % 16777216;
    i = (i ^ (i << 11)) % 16777216;
    i
}

#[derive(Default, Clone)]
pub struct Day22(Vec<Num>);

impl Day for Day22 {
    type Result1 = Num;
    type Result2 = Num;

    fn parse(&mut self, input: &str) {
        self.0 = input.lines().map(|line| line.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0
            .iter()
            .map(|&i| iterate(i, |i| next_secret(*i)).nth(2000).unwrap())
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        type SequenceMap = FxHashMap<Sequence, Num>;
        type Sequence = usize;
        fn to_sequence(diffs: &[i8], n: usize) -> Sequence {
            // diffs are in [-9, 9]
            const MAX: usize = 19;
            (0..4)
                .map(|i| diffs[(n + i) % 4])
                .map(|i| (i + 9) as usize)
                .reduce(|acc, i| (acc * MAX) + i)
                .unwrap()
        }

        let sequencemap = self
            .0
            .par_iter()
            .map(|&i| {
                let mut local_sequencemap: SequenceMap = Default::default();
                let mut diffs = [0i8; 4];
                let mut curr = i;
                for n in 0..=2000 {
                    let next = next_secret(curr);
                    let diff = (next % 10) as i8 - (curr % 10) as i8;
                    diffs[n % 4] = diff;
                    if n >= 3 {
                        let seq = to_sequence(&diffs, n - 3);
                        local_sequencemap.entry(seq).or_insert_with(|| next % 10);
                    }
                    curr = next;
                }
                local_sequencemap
            })
            .reduce(SequenceMap::default, |mut acc, it| {
                it.into_iter().for_each(|(k, v)| {
                    acc.entry(k)
                        .and_modify(|it| {
                            *it += v;
                        })
                        .or_insert(v);
                });
                acc
            });

        *sequencemap.values().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT1: &str = indoc! {"
        1
        10
        100
        2024
    "};

    const INPUT2: &str = indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn part_1() {
        let mut day: Day22 = Day22::default();
        day.parse(INPUT1);
        assert_eq!(day.part1(), 37327623);
    }

    #[test]
    fn part_2() {
        let mut day: Day22 = Day22::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 23);
    }
}
