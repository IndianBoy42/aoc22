use crate::utils::*;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|s| s.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let cals = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|s| s.parse::<usize>().unwrap()).sum());

    lazysort::SortedBy::sorted_by(cals, |a: &usize, b: &usize| b.cmp(a))
        .take(3)
        .sum()
}

#[test]
fn test() {
    let input = read_input("input1.txt").unwrap();
    assert_eq!(part1(&input), 72718);
    assert_eq!(part2(&input), 213089);
}
