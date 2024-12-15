//! # Day 15

use std::fmt::{Display, Write};

use aoc_runner::Day;
use fxhash::FxHashMap;
use fxhash::FxHashSet;

use crate::common::dir::Dir;

type Num = i16;
type Coord = (Num, Num);

#[derive(Clone)]
enum Box {
    Single,
    Left,
    Right,
}

#[derive(Default, Clone)]
struct Warehouse {
    walls: FxHashSet<Coord>,
    boxes: FxHashMap<Coord, Box>,
    robot: Coord,
}

impl Warehouse {
    fn from(value: &str, wide: bool) -> Self {
        let mut walls: FxHashSet<Coord> = Default::default();
        let mut boxes: FxHashMap<Coord, Box> = Default::default();
        let mut robot: Coord = Default::default();
        value.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| match ch {
                '#' => {
                    if !wide {
                        walls.insert((y as Num, x as Num));
                    } else {
                        walls.insert((y as Num, x as Num * 2));
                        walls.insert((y as Num, x as Num * 2 + 1));
                    }
                }
                'O' => {
                    if !wide {
                        boxes.insert((y as Num, x as Num), Box::Single);
                    } else {
                        boxes.insert((y as Num, x as Num * 2), Box::Left);
                        boxes.insert((y as Num, x as Num * 2 + 1), Box::Right);
                    }
                }
                '@' => {
                    if !wide {
                        robot = (y as Num, x as Num);
                    } else {
                        robot = (y as Num, x as Num * 2);
                    }
                }
                _ => {}
            })
        });
        Self {
            walls,
            boxes,
            robot,
        }
    }

    fn move_robot(&mut self, dir: Dir) {
        let next_pos = dir.go(self.robot);
        if self.boxes.contains_key(&next_pos) {
            if self.move_box(next_pos, dir, true) {
                self.robot = next_pos;
            }
        } else if !self.walls.contains(&next_pos) {
            self.robot = next_pos;
        }
    }

    fn move_box(&mut self, pos: Coord, dir: Dir, do_move: bool) -> bool {
        let next_pos = dir.go(pos);
        let other_pos = if matches!(dir, Dir::N | Dir::S) {
            match self.boxes.get(&pos) {
                Some(Box::Left) => Some(Dir::E.go(pos)),
                Some(Box::Right) => Some(Dir::W.go(pos)),
                _ => None,
            }
        } else {
            None
        };
        let other_next_pos = other_pos.map(|p| dir.go(p));

        // part of box was already moved when its counterpart was moved
        if !self.boxes.contains_key(&pos) {
            return true;
        }

        if self.walls.contains(&next_pos)
            || other_next_pos
                .map(|p| self.walls.contains(&p))
                .unwrap_or(false)
        {
            false
        } else if self.boxes.contains_key(&next_pos)
            || other_next_pos
                .map(|p| self.boxes.contains_key(&p))
                .unwrap_or(false)
        {
            if self.move_box(next_pos, dir, false)
                && other_next_pos
                    .map(|p| self.move_box(p, dir, false))
                    .unwrap_or(true)
            {
                self.move_box(next_pos, dir, do_move);
                if let Some(other_next_pos) = other_next_pos {
                    self.move_box(other_next_pos, dir, do_move);
                }

                if do_move {
                    if let Some(b) = self.boxes.remove(&pos) {
                        self.boxes.insert(next_pos, b);
                    }
                    if let Some(other_pos) = other_pos {
                        if let Some(b) = self.boxes.remove(&other_pos) {
                            self.boxes.insert(other_next_pos.unwrap(), b);
                        }
                    }
                }
                true
            } else {
                false
            }
        } else {
            if do_move {
                if let Some(b) = self.boxes.remove(&pos) {
                    self.boxes.insert(next_pos, b);
                }
                if let Some(other_pos) = other_pos {
                    if let Some(b) = self.boxes.remove(&other_pos) {
                        self.boxes.insert(other_next_pos.unwrap(), b);
                    }
                }
            }
            true
        }
    }

    fn box_gps((y, x): Coord) -> u32 {
        y as u32 * 100 + x as u32
    }

    fn box_gps_sum(&self) -> u32 {
        self.boxes
            .iter()
            .map(|(pos, b)| match b {
                Box::Single | Box::Left => Self::box_gps(*pos),
                Box::Right => 0,
            })
            .sum()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_y = self.walls.iter().map(|(y, _)| y).max().unwrap();
        let max_x = self.walls.iter().map(|(_, x)| x).max().unwrap();
        for y in 0..=*max_y {
            for x in 0..=*max_x {
                if self.walls.contains(&(y, x)) {
                    f.write_char('#')?;
                } else if self.boxes.contains_key(&(y, x)) {
                    match self.boxes.get(&(y, x)).unwrap() {
                        Box::Single => f.write_char('O')?,
                        Box::Left => f.write_char('[')?,
                        Box::Right => f.write_char(']')?,
                    }
                } else if self.robot == (y, x) {
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

#[derive(Default, Clone)]
pub struct Day15 {
    warehouse_str: String,
    moves: Vec<Dir>,
}

impl Day for Day15 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        let (warehouse, movements) = input.split_once("\n\n").unwrap();
        self.warehouse_str = warehouse.into();
        self.moves = movements
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| Dir::try_from(c).unwrap())
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut warehouse = Warehouse::from(&self.warehouse_str, false);
        for m in self.moves.iter() {
            warehouse.move_robot(*m);
        }
        warehouse.box_gps_sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        let mut warehouse = Warehouse::from(&self.warehouse_str, true);
        for m in self.moves.iter() {
            warehouse.move_robot(*m);
        }
        warehouse.box_gps_sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT1: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    const INPUT2: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn part_1() {
        {
            let mut day = Day15::default();
            day.parse(INPUT1);
            assert_eq!(day.part1(), 2028);
        }
        {
            let mut day = Day15::default();
            day.parse(INPUT2);
            assert_eq!(day.part1(), 10092);
        }
    }

    #[test]
    fn part_2() {
        let mut day = Day15::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 9021);
    }
}
