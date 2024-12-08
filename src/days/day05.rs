//! # Day 05

use aoc_runner::Day;
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

use crate::common::transform::Transform;

/// Rule that: [0] must be before [1] <=> [1] **must not** be before [0]
type Rule = (Page, Page);

/// A collection of rules
#[derive(Debug, Default, Clone)]
struct Rules {
    map: HashMap<Page, Vec<Page>>,
}

impl<T> From<T> for Rules
where
    T: IntoIterator<Item = Rule>,
{
    fn from(values: T) -> Self {
        let mut map: HashMap<Page, Vec<Page>> = HashMap::default();
        for (before, after) in values {
            map.entry(before)
                .and_modify(|v| {
                    v.push(after);
                })
                .or_insert(vec![after]);
        }

        Self { map }
    }
}

impl Rules {
    fn must_not_be_before(&self, page: Page) -> Option<impl Iterator<Item = &Page>> {
        self.map.get(&page).map(|v| v.iter())
    }
}

type Page = u16;
type Pages = Vec<Page>;

#[derive(Default, Clone)]
pub struct Day05 {
    rules: Rules,
    pages_list: Vec<Pages>,
}

impl Day for Day05 {
    type Result1 = u32;
    type Result2 = u32;

    fn parse(&mut self, input: &str) {
        let (rules, pages) = input.split_once("\n\n").unwrap();

        self.rules = rules
            .lines()
            .map(|line| {
                line.split_once("|")
                    .unwrap()
                    .transform(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            })
            .into();

        self.pages_list = pages
            .lines()
            .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
            .collect();
    }

    fn part1(&mut self) -> Self::Result1 {
        self.pages_list
            .iter()
            .filter(|pages| self.page_list_is_valid(pages))
            .map(|pages| *pages.get((pages.len() - 1) / 2).unwrap() as <Self as Day>::Result1)
            .sum()
    }

    fn part2(&mut self) -> Self::Result2 {
        self.pages_list
            .iter()
            .filter(|pages| !self.page_list_is_valid(pages))
            .map(|pages| self.reorder_pages(pages))
            .map(|pages| *pages.get((pages.len() - 1) / 2).unwrap() as <Self as Day>::Result1)
            .sum()
    }
}

impl Day05 {
    /// Checks if [pages] follows the given rules
    fn page_list_is_valid(&self, pages: &Pages) -> bool {
        pages.iter().enumerate().skip(1).all(|(idx, p)| {
            if let Some(mut not_befores) = self.rules.must_not_be_before(*p) {
                not_befores.all(|not_before| {
                    pages.iter().take(idx).filter(|a| *a == not_before).count() == 0
                })
            } else {
                true
            }
        })
    }

    /// Reorder [pages] to follow the given rules
    fn reorder_pages(&self, pages: &Pages) -> Pages {
        let mut v: Pages = Vec::with_capacity(pages.len());
        'outer: for p in pages {
            let not_befores = self
                .rules
                .must_not_be_before(*p)
                .map(|i| i.collect::<HashSet<&Page>>())
                .unwrap_or_default();

            for idx in 0..v.len() {
                if not_befores.contains(&v[idx]) {
                    v.insert(idx, *p);
                    continue 'outer;
                }
            }

            v.push(*p);
        }

        v
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part_1() {
        let mut day = Day05::default();
        day.parse(INPUT);
        assert_eq!(day.part1(), 143);
    }

    #[test]
    fn part_2() {
        let mut day = Day05::default();
        day.parse(INPUT);
        assert_eq!(day.part2(), 123);
    }
}
