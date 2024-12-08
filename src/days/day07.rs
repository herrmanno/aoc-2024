//! # Day 07

use aoc_runner::Day;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Default, Clone)]
struct Equation {
    result: u64,
    parameters: Vec<u64>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Concatenation {
    WithoutConcatenation,
    WithConcatenation,
}

impl Equation {
    fn is_valid(&self, concat: Concatenation) -> bool {
        let mut results: Vec<u64> = vec![self.parameters[0]];
        for p in &self.parameters[1..] {
            let mut next_results: Vec<u64> = Vec::with_capacity(results.len() * 2);
            for r in results.iter() {
                if let Some(n) = Some(r + p).take_if(|it| *it <= self.result) {
                    next_results.push(n);
                }
                if let Some(n) = Some(r * p).take_if(|it| *it <= self.result) {
                    next_results.push(n);
                }

                if matches!(concat, Concatenation::WithConcatenation) {
                    if let Some(n) = Some(r.concat(*p)).take_if(|it| *it <= self.result) {
                        next_results.push(n);
                    }
                }
            }
            results = next_results;
        }

        results.contains(&self.result)
    }
}

#[derive(Default, Clone)]
pub struct Day07(Vec<Equation>);

impl Day for Day07 {
    type Result1 = u64;
    type Result2 = u64;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| {
                let (result, parts) = line.split_once(": ").unwrap();
                Equation {
                    result: result.parse().unwrap(),
                    parameters: parts
                        .split_whitespace()
                        .map(|it| it.parse().unwrap())
                        .collect(),
                }
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0
            .par_iter()
            .filter(|eq| eq.is_valid(Concatenation::WithoutConcatenation))
            .map(|it| it.result)
            .sum::<<Self as Day>::Result1>()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0
            .par_iter()
            .filter(|eq| eq.is_valid(Concatenation::WithConcatenation))
            .map(|it| it.result)
            .sum::<<Self as Day>::Result1>()
    }
}

trait Concat {
    fn concat(self, rhs: Self) -> Self;
}

impl Concat for u64 {
    fn concat(self, rhs: Self) -> Self {
        // if self == 1 {
        //     return 10 + rhs;
        // }

        let factor10 = if rhs == 1 {
            10
        } else {
            10_u64.pow((rhs as f64).log10().ceil() as u32)
        };
        self * factor10 + rhs
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(1_u64.concat(1), 11);
        assert_eq!(1_u64.concat(2), 12);
        assert_eq!(9_u64.concat(1), 91);
        assert_eq!(10_u64.concat(1), 101);
        assert_eq!(99_u64.concat(1), 991);
        assert_eq!(100_u64.concat(1), 1001);
    }

    const INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn part_1() {
        let mut day = Day07::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 3749);
    }

    #[test]
    fn part_2() {
        let mut day = Day07::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 11387);
    }
}
