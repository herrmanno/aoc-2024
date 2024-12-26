//! # Day 24

use aoc_runner::Day;
use fxhash::FxHashMap;
use itertools::Itertools;

type Num = u64;

#[derive(Debug, Clone)]
enum Equation {
    Equation { lhs: String, rhs: String, op: Op },
    Const { value: bool },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn op(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Op::And => lhs && rhs,
            Op::Or => lhs || rhs,
            Op::Xor => lhs ^ rhs,
        }
    }
}

#[derive(Default, Clone)]
pub struct Day24 {
    equations: FxHashMap<String, Equation>,
}

impl Day for Day24 {
    type Result1 = Num;
    type Result2 = String;

    fn parse(&mut self, input: &str) {
        let (wires, equations) = input.split_once("\n\n").unwrap();
        for line in wires.lines() {
            self.equations.insert(
                line[0..3].to_string(),
                Equation::Const {
                    value: &line[5..6] == "1",
                },
            );
        }

        for line in equations.lines() {
            let words = line.split_whitespace().collect::<Vec<_>>();
            self.equations.insert(
                words[4].to_string(),
                Equation::Equation {
                    lhs: words[0].to_string(),
                    rhs: words[2].to_string(),
                    op: match words[1] {
                        "AND" => Op::And,
                        "OR" => Op::Or,
                        "XOR" => Op::Xor,
                        op => panic!("Bad op: {op}"),
                    },
                },
            );
        }
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut result = 0;
        for zwire in self.equations.keys().filter(|it| it.starts_with('z')) {
            let pos: usize = zwire[1..].parse().unwrap();
            let value = self.get_value(zwire) as Num;
            result |= value << pos;
        }
        result
    }

    fn part2(&mut self) -> Self::Result2 {
        let valid_and = |lhs: &str, rhs: &str, out: &str| {
            !out.starts_with('z') && {
                lhs == "x00"
                    || rhs == "x00"
                    || self.equations.iter().any(|(_, eq)| {
                        matches!(eq, Equation::Equation {
                            lhs,
                            rhs,
                            op: Op::Or,
                        } if lhs == out || rhs == out)
                    })
            }
        };
        let valid_or = |_lhs: &str, _rhs: &str, out: &str| out == "z45" || !out.starts_with('z');
        let valid_xor = |lhs: &str, rhs: &str, out: &str| {
            out.starts_with('z') || {
                lhs[1..].parse().unwrap_or(0) == rhs[1..].parse().unwrap_or(1) && {
                    self.equations.iter().any(|(_, eq)| {
                        matches!(eq, Equation::Equation {
                            lhs,
                            rhs,
                            op: Op::Xor,
                        } if lhs == out || rhs == out)
                    })
                }
            }
        };
        let faulty_wires = self
            .equations
            .iter()
            .filter(|(out, eq)| !match eq {
                Equation::Equation { lhs, rhs, op } => match op {
                    Op::And => valid_and(lhs, rhs, out),
                    Op::Or => valid_or(lhs, rhs, out),
                    Op::Xor => valid_xor(lhs, rhs, out),
                },
                _ => true,
            })
            .map(|it| it.0)
            .sorted()
            .cloned()
            .reduce(|a, b| format!("{a},{b}"))
            .unwrap();

        faulty_wires
    }
}

impl Day24 {
    fn get_value(&self, wire: &str) -> bool {
        match self.equations.get(wire).expect("Bad wire") {
            Equation::Equation { lhs, rhs, op } => op.op(self.get_value(lhs), self.get_value(rhs)),
            Equation::Const { value } => *value,
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn part_1() {
        let mut day: Day24 = Day24::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 2024);
    }
}
