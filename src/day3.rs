use std::{ops::ControlFlow, str::from_utf8};

use crate::utils::*;

/// Returns the common character in x, y
fn common<'a, I: Iterator<Item = char> + 'a>(x: I, y: &'a str) -> impl Iterator<Item = char> + 'a {
    // x.cartesian_product(y.chars())
    //     .filter(|(x, y)| x == y)
    //     .map(|(x, _)| x)
    x.filter(move |&x| y.contains(x))
    // x.filter(move |&x| y.bytes().any(|y| x as u8 == y))
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // Split line in half
            line.split_at(line.len() / 2)
        })
        .filter_map(|(l, r)| {
            // Get the common lettert from (l, r)
            common(l.chars(), r).next()
        })
        .map(prio)
        .sum()
}

fn prio(c: char) -> usize {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    if c.is_ascii_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .array_chunks()
        .map(|[a, b, c]| common(common(a.chars(), b), c).next().unwrap())
        .map(prio)
        .sum()
}

#[test]
fn test() {
    let input = read_input("input3.txt").unwrap();
    //     let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part1(&input), 7903);
    assert_eq!(part2(&input), 2548);
}
