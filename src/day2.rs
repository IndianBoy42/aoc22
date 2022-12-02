use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let (pos, dep) = input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(dir, len)| (dir, len.parse().unwrap()))
        .fold((0, 0), |(pos, dep), (dir, len): (&str, usize)| match dir {
            "forward" => (pos + len, dep),
            "up" => (pos, dep - len),
            "down" => (pos, dep + len),
            _ => unreachable!(),
        });
    pos * dep
}

pub fn part2(input: &str) -> usize {
    let (pos, dep, _) = input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(dir, len)| (dir, len.parse().unwrap()))
        .fold(
            (0, 0, 0),
            |(pos, dep, aim), (dir, len): (&str, usize)| match dir {
                "forward" => (pos + len, dep + aim * len, aim),
                "up" => (pos, dep, aim - len),
                "down" => (pos, dep, aim + len),
                _ => unreachable!(),
            },
        );
    pos * dep
}

#[test]
fn test() {
    let input = read_input("input2.txt").unwrap();
    assert_eq!(part1(&input), 1_690_020);
    assert_eq!(part2(&input), 1_408_487_760);
}
