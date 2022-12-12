use std::cell::{Cell, RefCell};

use crate::{
    grid::{all_neighbours, all_neighbours_if, Grid2D},
    searcher::{BFSearcher, DFSearcher},
    utils::*,
};
use lazysort::Sorted;
use regex::Regex;
use smallvec::SmallVec;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    TimesConst(u64),
    PlusConst(u64),
    TimesOld,
}

impl Operation {
    fn inspect(self, old: u64) -> u64 {
        match self {
            Operation::TimesConst(c) => old * c,
            Operation::PlusConst(c) => old + c,
            Operation::TimesOld => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((_, right)) = s.split_once('*') {
            if right.trim() == "old" {
                Ok(Operation::TimesOld)
            } else {
                Ok(Operation::TimesConst(right.trim().parse().unwrap()))
            }
        } else if let Some((_, right)) = s.split_once('+') {
            Ok(Operation::PlusConst(right.trim().parse().unwrap()))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<SmallVec<[u64; 10]>>,
    operation: Operation,
    divisibility: u64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new(block: &str) -> Option<Self> {
        let mut lines = block.lines();
        lines.next()?;
        let items = lines
            .next()?
            .split_at("  Starting items: ".len())
            .1
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        let operation = lines
            .next()?
            .split_at("  Operation: ".len())
            .1
            .parse()
            .ok()?;
        let divisibility = lines
            .next()?
            .split_at("  Test: divisibly by ".len())
            .1
            .parse()
            .ok()?;
        let if_true = lines
            .next()?
            .split_at("    If true: throw to monkey ".len())
            .1
            .parse()
            .ok()?;
        let if_false = lines
            .next()?
            .split_at("    If false: throw to monkey ".len())
            .1
            .parse()
            .ok()?;

        Some(Monkey {
            items: RefCell::new(items),
            operation,
            divisibility,
            if_true,
            if_false,
        })
    }

    fn inspect(&self, old: u64, div: u64) -> (u64, usize) {
        let new = self.operation.inspect(old) / div;
        (
            new,
            if new % self.divisibility == 0 {
                self.if_true
            } else {
                self.if_false
            },
        )
    }
    fn catch(&self, item: u64) {
        self.items.borrow_mut().push(item);
    }
}

pub fn part1(input: &str) -> usize {
    let monkeys = input
        .split("\n\n")
        .map(Monkey::new)
        .map(Option::unwrap)
        .collect_vec();
    let mut inspect_count = vec![0; monkeys.len()];

    for _ in 0..20 {
        for (monkey, i) in monkeys.iter().zip(&mut inspect_count) {
            *i += monkey.items.borrow().len();
            for item in monkey.items.replace(Default::default()) {
                let (item, throw_to) = monkey.inspect(item, 3);
                monkeys[throw_to].catch(item);
            }
        }

        // println!(
        //     "{}",
        //     monkeys
        //         .iter()
        //         .map(|m| m.items.borrow().iter().join(", "))
        //         .join("\n")
        // );
    }

    inspect_count.sort_unstable();
    inspect_count.iter().rev().take(2).product()
}

pub fn part2(input: &str) -> usize {
    let monkeys = input
        .split("\n\n")
        .map(Monkey::new)
        .map(Option::unwrap)
        .collect_vec();
    let mut inspect_count = vec![0; monkeys.len()];

    let total_div: u64 = monkeys.iter().map(|m| m.divisibility).product();

    for _ in 0..10000 {
        for (monkey, i) in monkeys.iter().zip(&mut inspect_count) {
            *i += monkey.items.borrow().len();
            for item in monkey.items.replace(Default::default()) {
                let (item, throw_to) = monkey.inspect(item, 1);
                monkeys[throw_to].catch(item % total_div);
            }
        }

        // println!(
        //     "{}",
        //     monkeys
        //         .iter()
        //         .map(|m| m.items.borrow().iter().join(", "))
        //         .join("\n")
        // );
    }

    inspect_count.sort_unstable();
    inspect_count.iter().rev().take(2).product()
}

#[test]
fn test() {
    let input = read_input("input11.txt").unwrap();
    assert_eq!(part1(&input), 55216);
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part2(&input), 12848882750);
}
