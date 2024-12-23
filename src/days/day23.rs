//! # Day 23
//!
//! Possible optimazations (for part 2):
//! Map node names to indices in a Vec<node>, so one doesn't have to clone Strings all the time

use std::fmt::{Display, Write};

use aoc_runner::Day;
use fxhash::{FxHashMap, FxHashSet};
type Node = String;
type Edge = (Node, Node);

#[derive(Debug, PartialEq, Eq)]
pub struct Clique(Vec<Node>);

impl Clique {
    fn from(mut nodes: Vec<Node>) -> Self {
        nodes.sort();
        Self(nodes)
    }
}

impl Display for Clique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0[0].as_str())?;
        for s in &self.0[1..] {
            f.write_char(',')?;
            f.write_str(s.as_str())?;
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Day23 {
    nodes: Vec<Node>,
    edges: FxHashSet<Edge>,
    neighbours: FxHashMap<Node, FxHashSet<Node>>,
}

impl Day23 {
    /// Find maximum clique
    ///
    /// see https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm#With_pivoting
    fn bron_kerbosch(
        &self,
        r: &mut Vec<Node>,
        p: &mut FxHashSet<Node>,
        x: &mut FxHashSet<Node>,
    ) -> Option<Vec<Node>> {
        if p.is_empty() && x.is_empty() {
            return Some(r.to_vec());
        }
        let pivot = {
            if !p.is_empty() {
                p.iter().next()
            } else {
                x.iter().next()
            }
        }
        .unwrap();

        let empty_set: FxHashSet<Node> = Default::default();

        let pivot_neighbors = self.neighbours.get(pivot).unwrap_or(&empty_set);
        let vertices = p
            .iter()
            .filter(|&it| !pivot_neighbors.contains(it))
            .cloned()
            .collect::<Vec<_>>();

        let mut max_clique: Option<Vec<Node>> = None;

        for v in vertices {
            let v_neighbours = self.neighbours.get(&v).unwrap_or(&empty_set);

            {
                r.push(v.clone());
                let mut p = p.intersection(v_neighbours).cloned().collect();
                let mut x = x.intersection(v_neighbours).cloned().collect();
                if let Some(result) = self.bron_kerbosch(r, &mut p, &mut x) {
                    if let Some(ref curr_max_clique) = max_clique {
                        if result.len() > curr_max_clique.len() {
                            max_clique = Some(result);
                        }
                    } else {
                        max_clique = Some(result);
                    }
                }
                r.pop();
            }
            p.remove(&v);
            x.insert(v.clone());
        }

        max_clique
    }
}

impl Day for Day23 {
    type Result1 = usize;
    type Result2 = Clique;

    fn parse(&mut self, input: &str) {
        let mut nodes: FxHashSet<Node> = Default::default();
        let mut edges: FxHashSet<Edge> = Default::default();
        let mut neighbours: FxHashMap<Node, FxHashSet<Node>> = Default::default();

        for line in input.lines() {
            let a = line[0..2].to_string();
            let b = line[3..5].to_string();
            nodes.insert(a.clone());
            nodes.insert(b.clone());
            edges.insert((a.clone(), b.clone()));
            edges.insert((b.clone(), a.clone()));
            neighbours.entry(a.clone()).or_default().insert(b.clone());
            neighbours.entry(b).or_default().insert(a);
        }

        self.nodes = nodes.into_iter().collect();
        self.nodes.sort();
        self.edges = edges;
        self.neighbours = neighbours;
    }

    fn part1(&mut self) -> Self::Result1 {
        self.nodes
            .iter()
            .filter(|n| n.starts_with('t'))
            .map(|n| {
                let mut sum = 0;
                if let Some(neighbours) = self.neighbours.get(n) {
                    for (i, a) in neighbours.iter().enumerate() {
                        if a.starts_with('t') && a < n {
                            continue;
                        }

                        for b in neighbours.iter().skip(i + 1) {
                            if b.starts_with('t') && b < n {
                                continue;
                            }

                            if self.edges.contains(&(a.clone(), b.clone())) {
                                sum += 1;
                            }
                        }
                    }
                    sum
                } else {
                    0
                }
            })
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        let result = self
            .bron_kerbosch(
                &mut Default::default(),
                &mut self.nodes.iter().cloned().collect(),
                &mut Default::default(),
            )
            .expect("No solution found");

        Clique::from(result)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn part_1() {
        let mut day: Day23 = Day23::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 7);
    }

    #[test]
    fn part_2() {
        let mut day: Day23 = Day23::default();
        day.parse(INPUT);
        assert_eq!(
            day.part2(),
            Clique::from(vec![
                String::from("co"),
                String::from("de"),
                String::from("ka"),
                String::from("ta")
            ])
        );
    }
}
