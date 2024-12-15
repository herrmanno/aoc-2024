//! # Day 14

use std::fmt::{Display, Write};

use aoc_runner::Day;
use bit_set::BitSet;
use fxhash::FxHashMap;

type Num = i32;

#[derive(Debug, Default, Clone)]
struct Robot {
    pos: (Num, Num),
    velocity: (Num, Num),
}

impl Robot {
    fn pos_at(&self, seconds: u32) -> (Num, Num) {
        (
            self.pos.0 + seconds as Num * self.velocity.0,
            self.pos.1 + seconds as Num * self.velocity.1,
        )
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (p, v) = value.split_once(" ").unwrap();
        let (px, py) = p[2..].split_once(",").unwrap();
        let (vx, vy) = v[2..].split_once(",").unwrap();
        let pos = (py.parse().unwrap(), px.parse().unwrap());
        let velocity = (vy.parse().unwrap(), vx.parse().unwrap());
        Self { pos, velocity }
    }
}

#[derive(Debug, Default, Clone)]
struct Robots<const H: usize, const W: usize>(Vec<Robot>);

impl<const H: usize, const W: usize> Robots<H, W> {
    fn iter(&self) -> impl Iterator<Item = &Robot> {
        self.0.iter()
    }
}

impl<const H: usize, const W: usize> Display for Robots<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=H {
            for x in 0..=W {
                if self.iter().any(|r| r.pos == (y as Num, x as Num)) {
                    f.write_char('@')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<const H: usize, const W: usize> From<&str> for Robots<H, W> {
    fn from(value: &str) -> Self {
        let robots = value.lines().map(Robot::from).collect();
        Self(robots)
    }
}

#[derive(Default, Clone)]
pub struct Day14<const H: usize = 103, const W: usize = 101>(Robots<H, W>);

impl<const H: usize, const W: usize> Day for Day14<H, W> {
    type Result1 = usize;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input.into();
    }

    fn part1(&mut self) -> Self::Result1 {
        let num_robots_per_quadrant = self
            .0
            .iter()
            .map(|r| r.pos_at(100))
            .map(|(y, x)| {
                (
                    y.rem_euclid(H as Num) as usize,
                    x.rem_euclid(W as Num) as usize,
                )
            })
            .fold([0; 4], |mut acc, pos| {
                #[allow(clippy::comparison_chain)]
                if pos.0 < ((H - 1) / 2) {
                    if pos.1 < ((W - 1) / 2) {
                        acc[0] += 1;
                    } else if pos.1 > ((W - 1) / 2) {
                        acc[1] += 1;
                    }
                } else if pos.0 > ((H - 1) / 2) {
                    if pos.1 < ((W - 1) / 2) {
                        acc[2] += 1;
                    } else if pos.1 > ((W - 1) / 2) {
                        acc[3] += 1;
                    }
                }
                acc
            });

        num_robots_per_quadrant
            .into_iter()
            .reduce(|a, b| a * b)
            .unwrap()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut poss: BitSet = BitSet::with_capacity(W * H);
        let mut numbers: Vec<usize> = Vec::with_capacity(self.0.iter().count());
        for i in 1..10000 {
            poss.clear();
            numbers.clear();

            self.0
                .iter()
                .map(|r| r.pos_at(i))
                .map(|(y, x)| {
                    (
                        y.rem_euclid(H as Num) as usize,
                        x.rem_euclid(W as Num) as usize,
                    )
                })
                .for_each(|(y, x)| {
                    poss.insert(y * H + x);
                    numbers.push(y * H + x);
                });

            let candidate = numbers.iter().any(|value| {
                (1..5).all(|diff| poss.contains(value + diff) && poss.contains(value + diff * H))
            });

            if candidate {
                return i;
            }
        }

        panic!("No solution found");
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn part_1() {
        let mut day = Day14::<7, 11>::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 12);
    }
}
