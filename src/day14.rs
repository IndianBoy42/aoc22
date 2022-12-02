use std::{str::from_utf8, string::String};

use crate::utils::*;

fn wrap<T, const N: usize>(t: T) -> ArrayVec<T, N> {
    let mut new = ArrayVec::new();
    new.push(t);
    new
}

pub fn solve(input: &str, n: usize) -> usize {
    let (template, input) = input.split_once("\n\n").unwrap();
    let rules = input
        .lines()
        .map(|line| line.split_once("->").unwrap())
        .map(|(left, right)| (left.trim(), right.trim()))
        .map(|(left, right)| {
            let (a, b) = left.bytes().collect_tuple().unwrap();
            let r = right.as_bytes()[0];
            // a.push(b);
            ([a, b], [[a, r], [r, b]])
        })
        .collect::<FMap<_, _>>();

    let template =
        template
            .as_bytes()
            .array_windows()
            .copied()
            .fold(fmap(rules.len()), |mut map, v| {
                map.entry(v).and_modify(|v| *v += 1).or_insert(1);
                map
            });

    let counts = (0..n).fold(template, |template, _| {
        let ret = fmap(template.len());
        template
            .into_iter()
            .flat_map(|(k, c)| {
                rules
                    .get(&k)
                    .copied()
                    // .unwrap_or_else(|| wrap::<_, 2>(k))
                    .unwrap()
                    .into_iter()
                    .map(move |v| (v, c))
            })
            .fold(ret, |mut map, (v, c)| {
                map.entry(v).and_modify(|v| *v += c).or_insert(c);
                map
            })
    });
    let &[fst, _] = counts.keys().next().unwrap();
    let mut counts1 = counts
        .into_iter()
        // .flat_map(|([a, b], c)| [(a, c), (b, c)])
        .map(|([_, b], c)| (b, c))
        .into_grouping_map()
        .sum();
    *counts1.get_mut(&fst).unwrap() += 1;

    match counts1.values().minmax() {
        itertools::MinMaxResult::MinMax(min, max) => max - min,
        itertools::MinMaxResult::NoElements | itertools::MinMaxResult::OneElement(_) => panic!(),
    }
}

pub fn part1(input: &str) -> usize {
    solve(input, 10)
}
pub fn part2(input: &str) -> usize {
    solve(input, 40)
}

#[test]
fn test() {
    let input = read_input("input14.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 2112);
    assert_eq!(part2(&input), 3_243_771_149_914);
}
