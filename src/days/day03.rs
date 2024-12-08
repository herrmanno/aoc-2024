//! # Day 03

use aoc_runner::Day;
use regex::Regex;

#[derive(Default, Clone)]
pub struct Day03(String);

const MUL_REG: &str = r#"(mul)\((\d+),(\d+)\)"#;
const DO_REG: &str = r#"(do)\(\)()()"#;
const DONT_REG: &str = r#"(don't)\(\)()()"#;

impl Day for Day03 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input.to_string();
    }

    fn part1(&mut self) -> Self::Result1 {
        let re = Regex::new(MUL_REG).unwrap();
        re.captures_iter(&self.0)
            .map(|c| c.extract())
            .map(|(_, [_, a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        let re = Regex::new(&format!("(?:{})|(?:{})|(?:{})", MUL_REG, DO_REG, DONT_REG)).unwrap();
        let mut sum = 0u32;
        let mut enabled = true;
        for (m, [a, b, c]) in re.captures_iter(&self.0).map(|c| c.extract()) {
            match a {
                "mul" => {
                    if enabled {
                        let b = b.parse::<u32>().unwrap();
                        let c = c.parse::<u32>().unwrap();
                        sum += b * c;
                    }
                }
                "do" => {
                    enabled = true;
                }
                "don't" => {
                    enabled = false;
                }
                _ => panic!("Bad regex match: {}", m),
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    const INPUT2: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn part_1() {
        let mut day = Day03::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 161);
    }

    #[test]
    fn part_2() {
        let mut day = Day03::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 48);
    }
}
