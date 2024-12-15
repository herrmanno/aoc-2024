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
    wide: bool,
}

impl Warehouse {
    fn widen(&mut self) {
        self.robot = (self.robot.0, self.robot.1 * 2);
        self.walls = self
            .walls
            .iter()
            .flat_map(|(y, x)| [(*y, x * 2), (*y, x * 2 + 1)])
            .collect();
        self.boxes = self
            .boxes
            .iter()
            .flat_map(|((y, x), _)| [((*y, x * 2), Box::Left), ((*y, x * 2 + 1), Box::Right)])
            .collect();
        self.wide = true;
    }

    fn move_robot(&mut self, dir: Dir) {
        let next_pos = dir.go(self.robot);
        // println!(
        //     "Move robot from ({}, {}) to ({}, {})",
        //     self.robot.0, self.robot.1, next_pos.0, next_pos.1
        // );
        if self.walls.contains(&next_pos) {
            return;
        } else if self.boxes.contains_key(&next_pos) {
            if self.move_box(next_pos, dir, true) {
                // println!("Move box");
                self.robot = next_pos;
            } else {
                return;
            }
        } else {
            self.robot = next_pos;
        }
    }

    fn move_box(&mut self, pos: Coord, dir: Dir, recurse: bool) -> bool {
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
            if self.can_move_box(next_pos, dir, true)
                && other_next_pos
                    .map(|p| self.can_move_box(p, dir, false))
                    .unwrap_or(true)
            {
                self.move_box(next_pos, dir, true);
                if let Some(other_next_pos) = other_next_pos {
                    self.move_box(other_next_pos, dir, true);
                }

                if let Some(b) = self.boxes.remove(&pos) {
                    self.boxes.insert(next_pos, b);
                }
                if let Some(other_pos) = other_pos {
                    if let Some(b) = self.boxes.remove(&other_pos) {
                        self.boxes.insert(other_next_pos.unwrap(), b);
                    }
                }
                true
            } else {
                false
            }
        } else {
            if let Some(b) = self.boxes.remove(&pos) {
                self.boxes.insert(next_pos, b);
            }
            if let Some(other_pos) = other_pos {
                if let Some(b) = self.boxes.remove(&other_pos) {
                    self.boxes.insert(other_next_pos.unwrap(), b);
                }
            }
            true
        }
    }

    fn can_move_box(&mut self, pos: Coord, dir: Dir, recurse: bool) -> bool {
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
            if self.can_move_box(next_pos, dir, true)
                && other_next_pos
                    .map(|p| self.can_move_box(p, dir, false))
                    .unwrap_or(true)
            {
                true
            } else {
                false
            }
        } else {
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

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let mut walls: FxHashSet<Coord> = Default::default();
        let mut boxes: FxHashMap<Coord, Box> = Default::default();
        let mut robot: Coord = Default::default();
        value.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| match ch {
                '#' => {
                    walls.insert((y as Num, x as Num));
                }
                'O' => {
                    boxes.insert((y as Num, x as Num), Box::Single);
                }
                '@' => {
                    robot = (y as Num, x as Num);
                }
                _ => {}
            })
        });
        Self {
            wide: false,
            walls,
            boxes,
            robot,
        }
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
    warehouse: Warehouse,
    moves: Vec<Dir>,
}

impl Day for Day15 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        let (warehouse, movements) = input.split_once("\n\n").unwrap();
        self.warehouse = warehouse.into();
        self.moves = movements
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| Dir::try_from(c).unwrap())
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        // println!("{}", self.warehouse);
        for m in self.moves.iter() {
            self.warehouse.move_robot(*m);
            // println!("{:?}", m);
            // println!("{}", self.warehouse);
        }
        self.warehouse.box_gps_sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.warehouse.widen();
        // println!("{}", self.warehouse);
        for m in self.moves.iter() {
            self.warehouse.move_robot(*m);
            // println!("{:?}", m);
            // println!("{}", self.warehouse);
        }
        // println!("{}", self.warehouse);
        self.warehouse.box_gps_sum()
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
