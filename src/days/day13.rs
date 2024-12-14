//! # Day 13

use aoc_runner::Day;

use crate::common::transform::Transform;

type Num = i64;

#[derive(Debug, Default, Clone)]
struct Machine {
    button_a: (Num, Num),
    button_b: (Num, Num),
    prize: (Num, Num),
}

impl Machine {
    fn get_num_buttons_to_win(&self) -> Option<(Num, Num)> {
        // see https://www.cuemath.com/geometry/intersection-of-two-lines/

        let a1 = self.button_a.0;
        let b1 = self.button_b.0;
        let c1 = -self.prize.0;
        let a2 = self.button_a.1;
        let b2 = self.button_b.1;
        let c2 = -self.prize.1;

        let x0 = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y0 = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

        // if there only exists a non-integer solution, the found x0/y0 values won't sum up
        // checking afterwards might be less costly than doing floating point calculations
        if x0 * self.button_a.0 + y0 * self.button_b.0 != self.prize.0 {
            return None;
        }
        if x0 * self.button_a.1 + y0 * self.button_b.1 != self.prize.1 {
            return None;
        }

        Some((x0, y0))
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let (line1, line2, line3) = value
            .lines()
            .transform(|mut it| (it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
        let button_a = line1
            .split_once(", Y+")
            .unwrap()
            .transform(|(a, b)| (a[12..].parse().unwrap(), b.parse().unwrap()));
        let button_b = line2
            .split_once(", Y+")
            .unwrap()
            .transform(|(a, b)| (a[12..].parse().unwrap(), b.parse().unwrap()));
        let prize = line3
            .split_once(", Y=")
            .unwrap()
            .transform(|(a, b)| (a[9..].parse().unwrap(), b.parse().unwrap()));
        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

#[derive(Default, Clone)]
pub struct Day13(Vec<Machine>);

impl Day for Day13 {
    type Result1 = u32;
    type Result2 = u64;

    fn parse(&mut self, input: &str) {
        self.0 = input.split("\n\n").map(Machine::from).collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0
            .iter()
            .filter_map(|m| m.get_num_buttons_to_win())
            .map(|(a, b)| (a * 3 + b) as <Self as Day>::Result1)
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0
            .iter_mut()
            .filter_map(|m| {
                m.prize = (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000);
                m.get_num_buttons_to_win()
            })
            .map(|(a, b)| (a * 3 + b) as <Self as Day>::Result2)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn part_1() {
        let mut day = Day13::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 480);
    }
}
