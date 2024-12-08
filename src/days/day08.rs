//! # Day 08

use aoc_runner::Day;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use itertools::iterate;

use crate::common::transform::Transform;

type Num = i16;
type Coord = (Num, Num);

#[derive(Default, Clone)]
pub struct Day08 {
    input: String,
    antennas_by_type: HashMap<char, Vec<Coord>>,
    size: (usize, usize),
}

impl Day for Day08 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.input = input.to_string();

        self.size = input
            .lines()
            .transform(|mut lines| (lines.clone().count(), lines.next().unwrap().chars().count()));

        self.antennas_by_type = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, ch)| match ch {
                        '.' => None,
                        ch => Some((ch, (y as Num, x as Num))),
                    })
            })
            .fold(Default::default(), |mut map, (ch, pos)| {
                map.entry(ch).or_default().push(pos);
                map
            })
    }

    fn part1(&mut self) -> Self::Result1 {
        self.solve(|a, b| self.get_antinodes(a, b)) as Self::Result1
    }

    fn part2(&mut self) -> Self::Result2 {
        self.solve(|a, b| self.get_antinodes_in_line(a, b)) as Self::Result2
    }
}

impl Day08 {
    fn solve<F, I>(&self, f: F) -> usize
    where
        I: Iterator<Item = Coord>,
        F: Fn(&Coord, &Coord) -> I,
    {
        self.antennas_by_type
            .iter()
            .flat_map(|(_, poss)| {
                let pairs = poss
                    .iter()
                    .enumerate()
                    .flat_map(|(idx, p)| poss.iter().skip(idx + 1).map(move |q| (p, q)));
                pairs.flat_map(|(a, b)| f(a, b))
            })
            .collect::<HashSet<_>>()
            .len()
    }

    fn get_antinodes(&self, a: &Coord, b: &Coord) -> impl Iterator<Item = Coord> + '_ {
        let slope = (a.0 - b.0, a.1 - b.1);
        let antinode1 = (a.0 + slope.0, a.1 + slope.1);
        let antinode2 = (b.0 - slope.0, b.1 - slope.1);
        [antinode1, antinode2]
            .into_iter()
            .filter(|pos| self.is_inside_map(pos))
    }

    fn get_antinodes_in_line(&self, a: &Coord, b: &Coord) -> impl Iterator<Item = Coord> + '_ {
        let slope = (a.0 - b.0, a.1 - b.1);
        let it1 = iterate(*a, move |(y, x)| (y + slope.0, x + slope.1))
            .take_while(|pos| self.is_inside_map(pos));
        let it2 = iterate(*b, move |(y, x)| (y - slope.0, x - slope.1))
            .take_while(|pos| self.is_inside_map(pos));
        it1.chain(it2)
    }

    fn is_inside_map(&self, a: &Coord) -> bool {
        a.0 >= 0 && a.0 < self.size.0 as Num && a.1 >= 0 && a.1 < self.size.1 as Num
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn part_1() {
        let mut day = Day08::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 14);
    }

    #[test]
    fn part_2() {
        let mut day = Day08::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 34);
    }
}
