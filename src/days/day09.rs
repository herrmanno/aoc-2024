//! # Day 09

use std::{
    collections::VecDeque,
    fmt::{Debug, Write},
};

use aoc_runner::Day;
use itertools::iterate;

type Num = u64;

enum File {
    File { id: Num, len: Num, pos: Num },
    Space { len: Num, pos: Num },
}

impl File {
    #[inline]
    fn len(&self) -> Num {
        match self {
            File::File { len, .. } => *len,
            File::Space { len, .. } => *len,
        }
    }

    #[inline]
    fn set_len(&mut self, size: Num) {
        match self {
            File::File { .. } => panic!("Cannot change length of file"),
            File::Space { len, .. } => {
                *len = size;
            }
        }
    }

    #[inline]
    fn pos(&self) -> Num {
        match self {
            File::File { pos, .. } => *pos,
            File::Space { pos, .. } => *pos,
        }
    }

    #[inline]
    fn set_pos(&mut self, size: Num) {
        match self {
            File::File { pos, .. } | File::Space { pos, .. } => {
                *pos = size;
            }
        }
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File { id, len, .. } => {
                let id = format!("{}", id);
                for _ in 0..*len {
                    f.write_str(&id)?;
                }
                Ok(())
            }
            Self::Space { len, .. } => {
                for _ in 0..*len {
                    f.write_char('.')?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Day09(Vec<Num>);

impl Day for Day09 {
    type Result1 = Num;
    type Result2 = Num;

    fn parse(&mut self, input: &str) {
        self.0 = input
            .chars()
            .filter_map(|it| it.to_digit(10))
            .map(|it| it as Num)
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        let mut vec: Vec<Option<Num>> = {
            let length = self.0.iter().sum::<Num>() as usize;
            let mut vec = Vec::with_capacity(length);

            for (i, &n) in self.0.iter().enumerate() {
                if i % 2 == 0 {
                    for _ in 0..n {
                        let i = (i >> 1) as Num;
                        vec.push(Some(i));
                    }
                } else {
                    for _ in 0..n {
                        vec.push(None);
                    }
                }
            }

            vec
        };

        let mut s = 0;
        let mut e = vec.len() - 1;
        while s < e {
            while vec[s].is_some() {
                s += 1;
            }
            while vec[e].is_none() {
                e -= 1;
            }

            vec.swap(s, e);
        }

        vec.into_iter()
            .flatten()
            .enumerate()
            .map(|(idx, n)| idx as Num * n)
            .sum::<<Self as Day>::Result1>()
    }

    fn part2(&mut self) -> Self::Result2 {
        let (mut files, mut spaces) = {
            let mut files: VecDeque<File> = Default::default();
            let mut spaces: VecDeque<File> = Default::default();
            let mut pos = 0;
            for (i, &len) in self.0.iter().enumerate() {
                if i % 2 == 0 {
                    let id = (i / 2) as Num;
                    files.push_back(File::File { id, len, pos });
                } else {
                    spaces.push_back(File::Space { len, pos });
                }
                pos += len;
            }
            (files, spaces)
        };

        'files: for file in files.iter_mut().rev() {
            for j in 0..spaces.len() {
                if spaces[j].pos() > file.pos() {
                    continue 'files;
                }

                if spaces[j].len() >= file.len() {
                    let diff = spaces[j].len() - file.len();
                    file.set_pos(spaces[j].pos());

                    if diff > 0 {
                        let space = spaces.get_mut(j).unwrap();
                        space.set_len(diff);
                        space.set_pos(space.pos() + file.len());
                    } else {
                        spaces.remove(j);
                    }

                    continue 'files;
                }
            }
        }

        files
            .into_iter()
            .filter_map(|it| match it {
                File::File { id, len, pos } => {
                    Some(iterate(pos, |it| it + 1).take(len as usize).sum::<Num>() * id)
                }
                File::Space { .. } => None,
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        let mut day = Day09::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 1928);
    }

    #[test]
    fn part_2() {
        let mut day = Day09::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 2858);
    }
}
