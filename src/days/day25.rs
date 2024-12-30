//! # Day 25

use aoc_runner::Day;

type Lock = [u8; 5];
type Key = [u8; 5];

#[derive(Default, Clone)]
pub struct Day25 {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

impl Day for Day25 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        fn parse_lock(s: &str) -> Lock {
            let mut lock = [0; 5];
            for (y, line) in s.lines().enumerate().skip(1) {
                for (x, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        lock[x] = y as u8;
                    }
                }
            }
            lock
        }

        fn parse_key(s: &str) -> Lock {
            let mut lock = [5; 5];
            for (y, line) in s.lines().enumerate().skip(1) {
                for (x, ch) in line.chars().enumerate() {
                    if ch == '.' {
                        lock[x] = 5 - y as u8;
                    }
                }
            }
            lock
        }

        input.split("\n\n").for_each(|block| {
            if block.starts_with("#") {
                self.locks.push(parse_lock(block));
            } else {
                self.keys.push(parse_key(block));
            }
        });
    }

    fn part1(&mut self) -> Self::Result1 {
        self.locks
            .iter()
            .map(|lock| {
                self.keys
                    .iter()
                    .filter(|key| lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5))
                    .count()
            })
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        0
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn part_1() {
        let mut day: Day25 = Day25::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 3);
    }
}
