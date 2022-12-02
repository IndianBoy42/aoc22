use std::ops::RangeInclusive;

use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let set = input
        .lines()
        .map(|line| dbg!(line))
        .map(|line| line.split_once(' ').unwrap())
        .map(|(cmd, line)| {
            let line = line
                .split(',')
                .map(|ch| ch.split_once('=').unwrap().1)
                .map(|ch| ch.split_once("..").unwrap())
                .map(|(l, r)| (l.parse::<i32>().unwrap()..=r.parse::<i32>().unwrap()))
                .collect::<ArrayVec<_, 3>>();
            let cmd = cmd == "on";
            (cmd, line)
        })
        .fold(fset(10000), |mut set, (cmd, range)| {
            let x = range[0].clone();
            let y = range[1].clone();
            let z = range[2].clone();
            let range = -50..=50;
            if range.contains(x.end())
                && range.contains(y.end())
                && range.contains(y.end())
                && range.contains(x.start())
                && range.contains(y.start())
                && range.contains(y.start())
            {
                for (i, j, k) in iproduct!(x, y, z) {
                    if cmd {
                        set.insert((i, j, k));
                    } else {
                        set.remove(&(i, j, k));
                    }
                }
            }
            set
        });

    let range = -50..=50;
    set.into_iter()
        .filter(|(i, j, k)| range.contains(i) && range.contains(j) && range.contains(k))
        .count()
}

fn overlap(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> RangeInclusive<i32> {
    // a.start() <= b.end() && b.start() <= a.end()
    let start = *a.start().max(b.start());
    let end = *a.end().min(b.end());
    start..=end
}
fn union(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> RangeInclusive<i32> {
    // a.start() <= b.start() && b.end() <= a.end()
    let start = *a.start().min(b.start());
    let end = *a.end().max(b.end());
    start..=end
}
fn contained(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && b.end() <= a.end()
}

pub fn part2(input: &str) -> usize {
    let set = input
        .lines()
        .map(|line| dbg!(line))
        .map(|line| line.split_once(' ').unwrap())
        .map(|(cmd, line)| {
            let line = line
                .split(',')
                .map(|ch| ch.split_once('=').unwrap().1)
                .map(|ch| ch.split_once("..").unwrap())
                .map(|(l, r)| (l.parse::<i32>().unwrap()..=r.parse::<i32>().unwrap()))
                .collect::<ArrayVec<_, 3>>();
            let cmd = cmd == "on";
            (cmd, line)
        })
        .fold(
            Vec::<ArrayVec<_, 3>>::with_capacity(1000),
            |mut ranges, (cmd, range)| {
                ranges.iter().filter_map(|prev| {
                    Some(
                        izip!(prev, range.clone())
                            .map(|(prev, range): (&RangeInclusive<_>, RangeInclusive<_>)| {
                                overlap(prev.clone(), range.clone())
                            })
                            .collect::<ArrayVec<_, 3>>(),
                    )
                    .filter(|overlaps| overlaps.iter().any(|s| !s.is_empty()))
                });
                ranges
            },
        );
    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input22.txt").unwrap();
    assert_eq!(part1(&input), 650099);
    assert_eq!(part2(&input), 0);
}
