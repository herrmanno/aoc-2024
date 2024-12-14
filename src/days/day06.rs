//! # Day 06

use std::sync::Arc;

use aoc_runner::Day;
use bit_set::BitSet;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::common::dir::Dir;

type Coord = (i16, i16);

#[derive(Default, Clone)]
pub struct Day06 {
    start_pos: Coord,
    obstacles: HashSet<Coord>,
    size: [i16; 2],
}

impl Day for Day06 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        self.size = {
            let y = input.lines().count();
            let x = input.lines().next().unwrap().chars().count();
            [y as i16, x as i16]
        };

        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((y as i16, x as i16), ch))
            })
            .for_each(|(pos, ch)| match ch {
                '#' => {
                    self.obstacles.insert(pos);
                }
                '^' => {
                    self.start_pos = pos;
                }
                _ => {}
            });
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut visited: HashSet<Coord> = Default::default();
        let mut direction = Dir::N;
        let mut pos = self.start_pos;
        loop {
            visited.insert(pos);
            let next_pos = direction.go(pos);

            if self.outside_map(&next_pos) {
                break;
            }

            if self.obstacles.contains(&next_pos) {
                direction = direction.turn_right();
            } else {
                pos = next_pos;
            }
        }
        visited.len() as <Self as Day>::Result1
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut direction = Dir::N;
        let mut visited: HashSet<Coord> = Default::default();
        let mut pos = self.start_pos;
        loop {
            visited.insert(pos);
            let next_pos = direction.go(pos);

            if self.outside_map(&next_pos) {
                break;
            }

            if self.obstacles.contains(&next_pos) {
                direction = direction.turn_right();
            } else {
                pos = next_pos;
            }
        }

        let jump_table = Arc::new(self.init_jump_table());

        visited
            .par_iter()
            .filter(|pos| self.walk(self.start_pos, Dir::N, **pos, Arc::clone(&jump_table)))
            .count() as <Self as Day>::Result2
    }
}

type JumpTable = HashMap<(Coord, Dir), Option<(Coord, Dir)>>;

impl Day06 {
    fn walk(
        &self,
        start: Coord,
        mut direction: Dir,
        obstacle: Coord,
        jump_table: Arc<JumpTable>,
    ) -> bool {
        let mut visited = BitSet::with_capacity(self.size[0] as usize * self.size[1] as usize * 4);

        let to_index = |pos: Coord, dir: Dir| {
            4 * (pos.0 as usize * self.size[0] as usize + pos.1 as usize)
                + (match dir {
                    Dir::N => 0,
                    Dir::S => 1,
                    Dir::W => 2,
                    Dir::E => 3,
                })
        };

        let mut pos = start;

        loop {
            let next_pos = direction.go(pos);

            if self.outside_map(&next_pos) {
                return false;
            }

            let visited_index = to_index(pos, direction);
            if visited.contains(visited_index) {
                return true;
            }
            visited.insert(visited_index);

            if self.obstacles.contains(&next_pos) {
                if pos.0 != obstacle.0 && pos.1 != obstacle.1 {
                    if let Some(new_state) = jump_table.get(&(pos, direction)) {
                        match new_state {
                            Some(new_state) => {
                                pos = new_state.0;
                                direction = new_state.1;
                                direction = direction.turn_right();
                                continue;
                            }
                            None => {
                                return false;
                            }
                        }
                    }
                }

                direction = direction.turn_right();
            } else if next_pos == obstacle {
                direction = direction.turn_right();
            } else {
                pos = next_pos;
            }
        }
    }

    fn init_jump_table(&self) -> JumpTable {
        let mut jump_table: JumpTable = Default::default();
        for &(y, x) in self.obstacles.iter() {
            let states = [
                ((y - 1, x), Dir::S),
                ((y + 1, x), Dir::N),
                ((y, x - 1), Dir::E),
                ((y, x + 1), Dir::W),
            ];
            for (mut pos, dir) in states.into_iter() {
                if self.outside_map(&pos) {
                    continue;
                }

                let start = (pos, dir);
                let dir = dir.turn_right();
                loop {
                    pos = dir.go(pos);
                    if self.outside_map(&pos) {
                        jump_table.insert(start, None);
                        break;
                    } else if self.obstacles.contains(&pos) {
                        jump_table.insert(start, Some((dir.opposite().go(pos), dir)));
                        break;
                    }
                }
            }
        }

        jump_table
    }

    #[inline]
    fn outside_map(&self, pos: &Coord) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size[0] || pos.1 >= self.size[1]
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part_1() {
        let mut day = Day06::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 41);
    }

    #[test]
    fn part_2() {
        let mut day = Day06::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 6);
    }
}
