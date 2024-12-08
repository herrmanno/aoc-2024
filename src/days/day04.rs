//! # Day 04

use aoc_runner::Day;
use fxhash::FxHashMap as HashMap;
use itertools::iterate;

const DIRS: [(i16, i16); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

const DIRS_X: [(i16, i16); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

#[derive(Default, Clone)]
pub struct Day04(HashMap<(i16, i16), char>);

impl Day for Day04 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((y as i16, x as i16), ch))
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let map = &self.0;

        (0..=max_y)
            .flat_map(move |y| (0..=max_x).map(move |x| (y, x)))
            .flat_map(|(y, x)| {
                DIRS.iter().filter(move |(dy, dx)| {
                    let found = iterate((y, x), |(y, x)| (y + dy, x + dx))
                        .take(4)
                        .map(|(y, x)| map.get(&(y, x)))
                        .collect::<Option<Vec<&char>>>();

                    matches!(found.as_deref(), Some(['X', 'M', 'A', 'S']))
                })
            })
            .count() as <Day04 as Day>::Result1
    }

    fn part2(&mut self) -> Self::Result2 {
        let max_y = self.max_y();
        let max_x = self.max_x();
        let map = &self.0;
        let mut amap: HashMap<(i16, i16), u8> = HashMap::default();

        (0..=max_y)
            .flat_map(move |y| (0..=max_x).map(move |x| (y, x)))
            .for_each(|(y, x)| {
                DIRS_X.iter().for_each(|(dy, dx)| {
                    let found = iterate((y, x), |(y, x)| (y + dy, x + dx))
                        .take(3)
                        .map(|(y, x)| map.get(&(y, x)))
                        .collect::<Option<Vec<&char>>>();

                    if let Some(['M', 'A', 'S']) = found.as_deref() {
                        let apos = (y + dy, x + dx);
                        amap.entry(apos)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                });
            });

        amap.into_iter().filter(|(_, count)| *count == 2).count() as <Day04 as Day>::Result1
    }
}

impl Day04 {
    fn max_y(&self) -> i16 {
        *self.0.keys().map(|(y, _)| y).max().unwrap()
    }

    fn max_x(&self) -> i16 {
        *self.0.keys().map(|(_, x)| x).max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ..X...
        .SAMX.
        .A..A.
        XMAS.S
        .X....
    "};

    const INPUT2: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part_1() {
        let mut day = Day04::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 4);

        let mut day = Day04::default();
        day.parse(INPUT2);
        assert_eq!(day.part1(), 18);
    }

    #[test]
    fn part_2() {
        let mut day = Day04::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 9);
    }
}
