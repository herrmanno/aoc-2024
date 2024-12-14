//! # Day 11G

use aoc_runner::Day;
use cached::proc_macro::cached;
use fxhash::FxHashMap;
use num::Integer;

type Num = u64;

#[derive(Clone)]
enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

#[cached]
fn blink(stone: Num) -> OneOrTwo<Num> {
    use OneOrTwo::*;

    if stone == 0 {
        One(1)
    } else {
        let s = stone.to_string();
        if s.len().is_even() {
            let (a, b) = s.split_at(s.len() >> 1);
            Two(a.parse().unwrap(), b.parse().unwrap())
        } else {
            One(stone * 2024)
        }
    }
}

#[derive(Default, Clone)]
struct Stones(FxHashMap<Num, usize>);

impl Stones {
    fn insert(&mut self, value: Num, count: usize) {
        let entry = self.0.entry(value).or_default();
        *entry += count;
    }

    fn blink(&mut self) {
        let mut old_stones: FxHashMap<Num, usize> = Default::default();
        std::mem::swap(&mut self.0, &mut old_stones);
        for (stone, count) in old_stones.into_iter() {
            match blink(stone) {
                OneOrTwo::One(a) => {
                    self.insert(a, count);
                }
                OneOrTwo::Two(a, b) => {
                    self.insert(a, count);
                    self.insert(b, count);
                }
            }
        }
    }

    fn num_stones(&self) -> usize {
        self.0.values().sum()
    }
}

#[derive(Default, Clone)]
pub struct Day11(Stones);

impl Day for Day11 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        for s in input.split_whitespace() {
            let num = s.parse().unwrap();
            self.0.insert(num, 1);
        }
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut stones = self.0.clone();
        for _ in 0..25 {
            stones.blink();
        }
        stones.num_stones()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut stones = self.0.clone();
        for _ in 0..75 {
            stones.blink();
        }
        stones.num_stones()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn part_1() {
        let mut day = Day11::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 55312);
    }
}
