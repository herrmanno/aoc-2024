//! # Day 17

use std::collections::VecDeque;

use aoc_runner::Day;
use itertools::Itertools;

type Num = u128;

#[derive(Debug)]
struct Machine {
    registers: [Num; 3],
}

impl Machine {
    fn execute(&mut self, code: &[Num]) -> Vec<Num> {
        let mut ip = 0;
        let mut out: Vec<Num> = Default::default();

        while let (Some(op), Some(operand)) = (code.get(ip), code.get(ip + 1)) {
            self.execute_op(*op, *operand, &mut ip, &mut out);
        }

        out
    }

    fn execute_op(&mut self, op: Num, operand: Num, ip: &mut usize, out: &mut Vec<Num>) {
        match op {
            0 => {
                self.registers[0] >>= self.get_combo_value(operand);
            }
            1 => {
                self.registers[1] ^= operand;
            }
            2 => {
                self.registers[1] = self.get_combo_value(operand) & 7;
            }
            3 => {
                if self.registers[0] != 0 {
                    *ip = operand as usize;
                    return;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                out.push(self.get_combo_value(operand) & 7);
            }
            6 => {
                self.registers[1] = self.registers[0] >> self.get_combo_value(operand);
            }
            7 => {
                self.registers[2] = self.registers[0] >> self.get_combo_value(operand);
            }
            _ => {
                unreachable!("Bad op {}", op);
            }
        }

        *ip += 2;
    }

    fn get_combo_value(&self, operand: Num) -> Num {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            _ => {
                panic!("Bad combo operand {}", operand);
            }
        }
    }
}

#[derive(Debug)]
pub struct Out(Vec<Num>);

impl std::fmt::Display for Out {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.iter().map(|it| it.to_string()).join(",");
        f.write_str(&s)
    }
}

#[derive(Default, Clone)]
pub struct Day17 {
    registers: [Num; 3],
    code: Vec<Num>,
}

impl Day for Day17 {
    type Result1 = Out;
    type Result2 = u128;

    fn parse(&mut self, input: &str) {
        let mut lines = input.lines();
        self.registers = [
            lines.next().unwrap()[12..].parse().unwrap(),
            lines.next().unwrap()[12..].parse().unwrap(),
            lines.next().unwrap()[12..].parse().unwrap(),
        ];
        self.code = lines.nth(1).unwrap()[9..]
            .split(",")
            .map(|it| it.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut machine = Machine {
            registers: self.registers,
        };
        Out(machine.execute(&self.code))
    }

    /// Essentially, every loop of the program does the following:
    /// ```text
    /// B = (A & 7) ^ 3 ^ (A >> ((A & 7) ^ 3)) ^ 5
    /// out(B)
    /// A /= 8
    ///```
    /// Meaning the output value is only affected by the ten lowest bits of A at any time.
    ///
    /// Therefore building the register value starting with the most significant bits and
    /// appending chunks of three bits allows for finding the smallest result.
    fn part2(&mut self) -> Self::Result2 {
        fn run(registers: [Num; 3], code: &[Num], l: usize) -> bool {
            let mut machine = Machine { registers };
            let out = machine.execute(code);
            out.into_iter()
                .rev()
                .zip(code.iter().rev())
                .take(l)
                .all(|(a, b)| a == *b)
        }

        fn test(registers: [Num; 3], code: &[Num], l: usize) -> bool {
            let mut machine = Machine { registers };
            let out = machine.execute(code);
            out.len() == l && run(registers, code, l)
        }

        type State = (Num, usize, usize);
        let mut agenda: VecDeque<State> = Default::default();

        let l = 1;
        let i = self.code.len() - 1;
        for ref candidate in 0..8 {
            agenda.push_back((*candidate, l, i));
        }

        while let Some((n, l, i)) = agenda.pop_front() {
            let registers = [n, self.registers[1], self.registers[2]];
            if run(registers, &self.code, l) {
                if l == self.code.len() {
                    if test(registers, &self.code, l) {
                        return n;
                    } else {
                        continue;
                    }
                }

                for candidate in 0..8 {
                    let new_n = (n << 3) + candidate;
                    agenda.push_back((new_n, l + 1, i - 1));
                }
            }
        }

        panic!("No solution found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    const INPUT2: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    fn part_1() {
        let mut day = Day17::default();
        day.parse(INPUT1);
        assert_eq!(day.part1().0, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn part_2() {
        let mut day = Day17::default();
        day.parse(INPUT2);
        assert_eq!(day.part2(), 117440);
    }
}
