//! # Day 14

use std::fmt::{Display, Write};

use aoc_runner::Day;

use crate::common::chinese_remainder::chinese_remainder;

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

    /// # Part 2
    ///
    /// As all robot's x-values repeat with a cycle length of W and all robot's y-values repeat with
    /// a cylce length of H, find the cycle offset, that indicates a anomaly for the x and the y
    /// dimension indepentently and calclutate the moment those pattern appear together via
    /// chinese reminder theorem.
    ///
    /// Hint: this only works if H and W are prime.
    fn part2(&mut self) -> Self::Result2 {
        let mut y_counts = [0; H];
        let mut x_counts = [0; W];
        let mut y_iter = (0, 0);
        let mut x_iter = (0, 0);

        for i in 1..W.max(H) {
            for n in y_counts.iter_mut() {
                *n = 0;
            }
            for n in x_counts.iter_mut() {
                *n = 0;
            }

            self.0
                .iter()
                .map(|r| r.pos_at(i as u32))
                .map(|(y, x)| {
                    (
                        y.rem_euclid(H as Num) as usize,
                        x.rem_euclid(W as Num) as usize,
                    )
                })
                .for_each(|(y, x)| {
                    y_counts[y] += 1;
                    x_counts[x] += 1;
                });

            let y_max = y_counts.iter().max().unwrap();
            if *y_max > y_iter.1 {
                y_iter = (i, *y_max);
            }
            let x_max = x_counts.iter().max().unwrap();
            if *x_max > x_iter.1 {
                x_iter = (i, *x_max);
            }
        }

        chinese_remainder(&[y_iter.0 as i64, x_iter.0 as i64], &[H as i64, W as i64]).unwrap()
            as <Self as Day>::Result2
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
