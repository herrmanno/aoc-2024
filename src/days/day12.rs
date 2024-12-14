//! # Day 12

use std::collections::VecDeque;

use aoc_runner::Day;
use fxhash::{FxHashMap, FxHashSet};

use crate::common::dir::Dir;

type C = i16;
type Coord = (C, C);

#[derive(Default, Clone)]
struct Garden(FxHashMap<Coord, char>);

impl Garden {
    fn areas(&self) -> Vec<Area> {
        let mut areas: Vec<Area> = Default::default();
        let mut coords_to_skip: FxHashSet<Coord> = Default::default();
        let coords = self.0.keys();

        for coord in coords {
            if coords_to_skip.contains(coord) {
                continue;
            }

            let mut area = Area {
                plant: *self.0.get(coord).unwrap(),
                fields: Default::default(),
            };

            let mut visited: FxHashSet<Coord> = Default::default();
            let mut agenda = VecDeque::from([*coord]);
            while let Some(coord @ (y, x)) = agenda.pop_front() {
                if !visited.insert(coord) {
                    continue;
                }

                if let Some(plant) = self.0.get(&coord) {
                    if *plant == area.plant {
                        area.insert(coord);
                        coords_to_skip.insert(coord);

                        for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            agenda.push_back((y + dy, x + dx));
                        }
                    }
                }
            }

            areas.push(area);
        }

        areas
    }
}

#[derive(Debug, Default, Clone)]
struct Area {
    plant: char,
    fields: FxHashSet<Coord>,
}

impl Area {
    fn insert(&mut self, field: Coord) {
        self.fields.insert(field);
    }

    fn area(&self) -> u32 {
        self.fields.len() as u32
    }

    fn perimeter(&self) -> u32 {
        self.fields
            .iter()
            .flat_map(|&(y, x)| [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)])
            .filter(|coord| !self.fields.contains(coord))
            .count() as u32
    }

    /// Equals number of edges
    fn sides(&self) -> u32 {
        let dir_pairs = Dir::ALL
            .iter()
            .zip(Dir::ALL.iter().skip(1).chain(Some(&Dir::ALL[0])))
            .collect::<Vec<_>>();

        self.fields
            .iter()
            .map(|&coord| {
                let outbound_edges = dir_pairs
                    .iter()
                    .filter(|(d1, d2)| {
                        !self.fields.contains(&d1.go(coord)) && !self.fields.contains(&d2.go(coord))
                    })
                    .count();

                let inbound_edges = dir_pairs
                    .iter()
                    .filter(|(d1, d2)| {
                        self.fields.contains(&d1.go(coord))
                            && self.fields.contains(&d2.go(coord))
                            && !self.fields.contains(&d1.go(d2.go(coord)))
                    })
                    .count();

                (outbound_edges + inbound_edges) as u32
            })
            .sum()
    }

    fn price(&self) -> u32 {
        self.area() * self.perimeter()
    }

    fn discount_price(&self) -> u32 {
        self.area() * self.sides()
    }
}

#[derive(Default, Clone)]
pub struct Day12(Garden);

impl Day for Day12 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        let garden = Garden(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, ch)| ((y as C, x as C), ch))
                })
                .collect(),
        );
        self.0 = garden;
    }

    fn part1(&mut self) -> Self::Result1 {
        self.0.areas().iter().map(Area::price).sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.0.areas().iter().map(Area::discount_price).sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const INPUT2: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};

    const INPUT3: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn part_1() {
        {
            let mut day = Day12::default();
            day.parse(INPUT);
            assert_eq!(day.part1(), 140);
        }
        {
            let mut day = Day12::default();
            day.parse(INPUT2);
            assert_eq!(day.part1(), 772);
        }
        {
            let mut day = Day12::default();
            day.parse(INPUT3);
            assert_eq!(day.part1(), 1930);
        }
    }

    #[test]
    fn part_2() {
        {
            let mut day = Day12::default();
            day.parse(INPUT);
            assert_eq!(day.part2(), 80);
        }
        {
            let mut day = Day12::default();
            day.parse(INPUT2);
            assert_eq!(day.part2(), 436);
        }
        {
            let mut day = Day12::default();
            day.parse(INPUT3);
            assert_eq!(day.part2(), 1206);
        }
    }
}
