use crate::utils::*;

pub fn part1(input: &str) -> isize {
    let pos = input
        .trim()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        // .sorted()
        .collect_vec();

    if true {
        let mut pos = pos;
        let med = pos.len() / 2;
        let median = if false {
            pos.sort_unstable();
            pos[med]
        } else {
            // O(n) and fast
            *pos.select_nth_unstable(med).1
        };

        pos.iter().map(|x| (x - median).abs()).sum::<isize>() as isize
    } else {
        let test = |tar: isize| pos.iter().map(|x| (x - tar).abs()).sum::<isize>() as isize;
        pos.iter().copied().map(test).min().unwrap()
    }
}

pub fn part2(input: &str) -> isize {
    let pos = input
        .trim()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        // .sorted()
        .collect_vec();

    // The minimum within +/- 1 of the mean
    let sum: isize = pos.iter().copied().sum();
    let len = pos.len() as isize;
    let hi = (sum + len - 1) / len;
    let lo = sum / len;

    let test = |tar: isize| {
        pos.iter()
            .map(|x| (x - tar).abs())
            .map(|d| d * (d + 1) / 2)
            .sum::<isize>() as isize
    };

    // Technically requires checking 3 values if the mean is an exact integer
    [lo, hi].into_iter().map(test).min().unwrap()
}

#[test]
fn test() {
    let input = read_input("input7.txt").unwrap();
    assert_eq!(part2(&input), 104_149_091);
    assert_eq!(part1(&input), 364_898);
}
