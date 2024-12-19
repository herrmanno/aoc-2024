//! # Day 19

use aoc_runner::Day;
use cached::proc_macro::cached;
use fxhash::FxHashSet;

type Towel = String;
type Design = String;

#[derive(Default, Clone)]
pub struct Day19 {
    towels: FxHashSet<Towel>,
    designs: Vec<Design>,
}

impl Day for Day19 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        let (towels, designs) = input.split_once("\n\n").unwrap();
        self.towels = towels.split(", ").map(str::to_owned).collect();
        self.designs = designs.lines().map(str::to_owned).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.designs
            .iter()
            .filter(|d| can_design(&self.towels, d))
            .count()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.designs
            .iter()
            .map(|d| num_designs(&self.towels, d))
            .sum()
    }
}

#[cached(convert = r#"{design.to_string()}"#, key = "String")]
fn can_design(towels: &FxHashSet<Towel>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for i in (1..=design.len()).rev() {
        if towels.contains(&design[0..i]) && can_design(towels, &design[i..]) {
            return true;
        }
    }

    false
}

#[cached(convert = r#"{design.to_string()}"#, key = "String")]
fn num_designs(towels: &FxHashSet<Towel>, design: &str) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut sum = 0;
    for i in (1..=design.len()).rev() {
        if towels.contains(&design[0..i]) {
            sum += num_designs(towels, &design[i..]);
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part_1() {
        let mut day = Day19::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 6);
    }

    #[test]
    fn part_2() {
        let mut day = Day19::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 16);
    }
}
