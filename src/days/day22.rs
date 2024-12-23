//! # Day 22

use aoc_runner::Day;
use itertools::iterate;

type Num = u64;

#[inline]
fn next_secret(mut i: Num) -> Num {
    const MASK: Num = 16777216 - 1;
    i = (i ^ (i << 6)) & MASK;
    i = (i ^ (i >> 5)) & MASK;
    i = (i ^ (i << 11)) & MASK;
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
        const MAX_SEQUENCE: usize = ((19 as Num).pow(4) - 1) as usize;
        type SequenceMap = Vec<Num>;
        type Sequence = usize;

        #[inline]
        fn to_sequence(diffs: &[i8], n: usize) -> Sequence {
            // diffs are in [-9, 9]
            const MAX: usize = 19;
            (0..4)
                .map(|i| diffs[(n + i) % 4])
                .map(|i| (i + 9) as usize)
                .reduce(|acc, i| (acc * MAX) + i)
                .unwrap()
        }

        let mut max = 0;
        let mut sequencemap: SequenceMap = vec![0; MAX_SEQUENCE];
        let mut seen = vec![false; MAX_SEQUENCE];
        for &i in self.0.iter() {
            seen.clear();
            seen.resize(MAX_SEQUENCE, false);

            let mut diffs = [0i8; 4];
            let mut curr = i;
            for n in 0..3 {
                let next = next_secret(curr);
                let diff = (next % 10) as i8 - (curr % 10) as i8;
                diffs[n % 4] = diff;
                curr = next;
            }
            for n in 3..=2000 {
                let next = next_secret(curr);
                let diff = (next % 10) as i8 - (curr % 10) as i8;
                diffs[n % 4] = diff;
                let seq = to_sequence(&diffs, n - 3);
                if !seen[seq] {
                    seen[seq] = true;
                    sequencemap[seq] += next % 10;
                    max = max.max(sequencemap[seq]);
                }
                curr = next;
            }
        }

        max
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
