use std::{
    ops::{ControlFlow, Range},
    str::from_utf8,
};

use crate::utils::*;

pub fn part1(input: &str) -> usize {
    // Input:
    // 2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8
    //
    // Parse the list as pair of ranges and find which lines have one range fully contained in the other
    fn parse_range(rng: &str) -> Range<u32> {
        rng.split_once('-')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .map(|(a, b)| a..b)
            .unwrap()
    }
    fn contained(a: &Range<u32>, b: &Range<u32>) -> bool {
        a.start >= b.start && a.end <= b.end
    }

    input
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .filter(|(l, r)| contained(l, r) || contained(r, l))
        .count()
}

pub fn part2(input: &str) -> usize {
    // Input:
    // 2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8
    //
    // Parse the list as pair of ranges and find which lines have one range fully contained in the other
    fn parse_range(rng: &str) -> Range<u32> {
        rng.split_once('-')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .map(|(a, b)| a..b)
            .unwrap()
    }
    fn overlap(a: &Range<u32>, b: &Range<u32>) -> bool {
        a.start <= b.start && a.end >= b.start || b.start <= a.start && b.end >= a.start
    }

    input
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(left, right)| (parse_range(left), parse_range(right)))
        .filter(|(l, r)| overlap(l, r))
        .count()
}

#[test]
fn test() {
    let input = read_input("input4.txt").unwrap();
    assert_eq!(part1(&input), 576);
    assert_eq!(part2(&input), 905);
}
