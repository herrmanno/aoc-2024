//! # Day 21

use std::{
    fmt::{Display, Write},
    hash::Hash,
    iter::repeat_n,
};

use aoc_runner::Day;
use fxhash::FxHashMap;
use itertools::Itertools;

use crate::common::{dir::Dir, transform::Transform};

type Num = i8;
type Coord = (Num, Num);
type Path = Vec<Key<Dir>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Key<T> {
    Key(T),
    Activate,
}

impl Display for Key<Dir> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Key::Key(d) => match d {
                Dir::N => '^',
                Dir::S => 'v',
                Dir::W => '<',
                Dir::E => '>',
            },
            Key::Activate => 'A',
        };
        f.write_char(c)
    }
}

#[allow(unused)]
struct KeyPath<T>(Vec<Key<T>>);

impl Display for KeyPath<Dir> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for k in self.0.iter() {
            k.fmt(f)?;
        }
        Ok(())
    }
}

trait Keypad<T> {
    fn key_to_pos(k: Key<T>) -> Coord;

    fn is_invalid_pos(pos: Coord) -> bool;

    fn shortest_paths_for_sequence(seq: &[Key<T>]) -> Vec<Path>
    where
        T: Copy + Eq + Hash,
    {
        let mut all_paths: Vec<Path> = vec![vec![]];
        for i in 0..(seq.len() - 1) {
            let paths =
                Self::valid_shortest_paths(Self::key_to_pos(seq[i]), Self::key_to_pos(seq[i + 1]));
            let mut new_all_paths: Vec<Path> = vec![];
            for p in all_paths {
                for q in paths.iter() {
                    let new_path = {
                        let mut p = p.clone();
                        p.extend_from_slice(q.as_slice());
                        p
                    };
                    new_all_paths.push(new_path);
                }
            }
            all_paths = new_all_paths;
        }
        all_paths.into_iter().dedup().collect()
    }

    fn valid_shortest_paths(from: Coord, to: Coord) -> Vec<Path> {
        Self::all_shortest_paths(from, to)
            .into_iter()
            .filter(|p| {
                let mut pos = from;
                for d in p {
                    pos = d.go(pos);
                    if Self::is_invalid_pos(pos) {
                        return false;
                    }
                }
                true
            })
            .map(|path| {
                path.into_iter()
                    .map(Key::Key)
                    .chain(Some(Key::Activate))
                    .collect()
            })
            .collect()
    }

    fn all_shortest_paths(from: Coord, to: Coord) -> Vec<Vec<Dir>> {
        use std::cmp::Ordering::*;
        let y_movement = match to.0.cmp(&from.0) {
            Less => Dir::N,
            Equal => Dir::N,
            Greater => Dir::S,
        };
        let x_movement = match to.1.cmp(&from.1) {
            Less => Dir::W,
            Equal => Dir::W,
            Greater => Dir::E,
        };

        let mut p = vec![
            repeat_n(y_movement, from.0.abs_diff(to.0) as usize)
                .chain(repeat_n(x_movement, from.1.abs_diff(to.1) as usize))
                .collect(),
            repeat_n(x_movement, from.1.abs_diff(to.1) as usize)
                .chain(repeat_n(y_movement, from.0.abs_diff(to.0) as usize))
                .collect(),
        ];
        p.dedup();
        p
    }
}

struct NumericKeypad;

impl Keypad<char> for NumericKeypad {
    fn key_to_pos(k: Key<char>) -> Coord {
        use self::Key::*;
        match k {
            Activate => (3, 2),
            Key('0') => (3, 1),
            Key('1') => (2, 0),
            Key('2') => (2, 1),
            Key('3') => (2, 2),
            Key('4') => (1, 0),
            Key('5') => (1, 1),
            Key('6') => (1, 2),
            Key('7') => (0, 0),
            Key('8') => (0, 1),
            Key('9') => (0, 2),
            _ => panic!(),
        }
    }

    fn is_invalid_pos(pos: Coord) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= 4 || pos.1 >= 3 || pos == (3, 0)
    }
}

fn robot_moving(from: Key<Dir>, to: Key<Dir>) -> Vec<Dir> {
    use self::Key::*;
    use Dir::*;
    // empirically 'best' movements. Where 'alternative' moves are possible, the best move,
    // based on trial an error, is chosen.
    match (from, to) {
        (Key(N), Key(W)) => vec![S, W],
        (Key(N), Key(E)) => vec![S, E], //alternatives
        (Key(N), Key(S)) => vec![S],
        (Key(S), Key(W)) => vec![W],
        (Key(S), Key(E)) => vec![E],
        (Key(S), Key(N)) => vec![N],
        (Key(W), Key(N)) => vec![E, N],
        (Key(W), Key(E)) => vec![E, E],
        (Key(W), Key(S)) => vec![E],
        (Key(E), Key(N)) => vec![W, N], //alternatives
        (Key(E), Key(W)) => vec![W, W],
        (Key(E), Key(S)) => vec![W],
        (Key(N), Activate) => vec![E],
        (Key(S), Activate) => vec![N, E], //alternative
        (Key(W), Activate) => vec![E, E, N],
        (Key(E), Activate) => vec![N],
        (Activate, Key(N)) => vec![W],
        (Activate, Key(S)) => vec![W, S], //alternative
        (Activate, Key(W)) => vec![S, W, W],
        (Activate, Key(E)) => vec![S],
        _ => vec![],
    }
}

#[derive(Default, Clone)]
pub struct Day21(Vec<Vec<Key<char>>>);
impl Day21 {
    fn solve(&self, num_pads: usize) -> usize {
        let mut result = 0;
        for mut code in self.0.iter().cloned() {
            let numeric_part = code
                .iter()
                .filter_map(|it| match it {
                    Key::Key(it) => Some(it),
                    Key::Activate => None,
                })
                .skip_while(|&&it| it == '0')
                .take_while(|&&it| it.is_numeric())
                .collect::<String>()
                .transform(|it| it.parse::<usize>().unwrap());

            code.insert(0, Key::Activate);

            // Cost for this code
            let code_cost: usize = code
                .into_iter()
                .tuple_windows()
                // for each tuple (window) of the code...
                .map(|(a, b)| {
                    // calculate the paths (there may be multiple) on the numeric keyboard
                    let numeric_paths = NumericKeypad::shortest_paths_for_sequence(&[a, b]);
                    // for each path on the numeric keyboard
                    numeric_paths
                        .into_iter()
                        .map(|numeric_path| {
                            /// Map from (from_key, to_key) to count of that moves in sequence
                            type MoveMap = FxHashMap<(Key<Dir>, Key<Dir>), usize>;

                            // turn numeric path to directional path,
                            let mut num_combos: MoveMap = Default::default();

                            // and store only the tuples (windows) of that path
                            for (&a, &b) in numeric_path.iter().tuple_windows() {
                                *num_combos.entry((a, b)).or_insert(0) += 1;
                            }

                            // also insert an extra 'start tuple': the first key on the
                            // directional keyboard must be reached from the 'A'
                            let a = numeric_path.into_iter().next().unwrap();
                            *num_combos.entry((Key::Activate, a)).or_insert(0) += 1;

                            // for every other directional keypad
                            for _ in 0..num_pads {
                                let mut new_num_combos: MoveMap = Default::default();
                                // take the movements (combos) needed on the current directional keypad
                                for ((a, b), count) in num_combos.iter() {
                                    // for every tuple
                                    for (a, b) in [Key::Activate]
                                        .into_iter()
                                        // calculate how that tuple can be achieved by movements on
                                        // the 'upper' directional keyboard
                                        .chain(robot_moving(*a, *b).into_iter().map(Key::Key))
                                        .chain([Key::Activate])
                                        .tuple_windows()
                                    {
                                        *new_num_combos.entry((a, b)).or_insert(0) += count;
                                    }
                                }
                                num_combos = new_num_combos;
                            }

                            // the number of single movements on the outer most directional keypad
                            // equals the number of movement-tuples ('combos')
                            num_combos.values().sum::<usize>()
                        })
                        // take the best path
                        .min()
                        .unwrap()
                })
                .sum();

            result += code_cost * numeric_part;
        }
        result
    }
}

impl Day for Day21 {
    type Result1 = usize;
    type Result2 = usize;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == 'A' { Key::Activate } else { Key::Key(c) })
                    .collect()
            })
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.solve(2)
    }

    fn part2(&mut self) -> Self::Result2 {
        self.solve(25)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part_1() {
        let mut day: Day21 = Day21::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 126384);
    }

    #[test]
    fn part_2() {
        let mut day: Day21 = Day21::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 154115708116294);
    }
}
