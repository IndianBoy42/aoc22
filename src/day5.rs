use std::{mem::size_of_val, ops::ControlFlow, str::from_utf8};

use crate::utils::*;

// TODO: maybe use complex numbers can make the iterations cleaner?
// TODO: use a contiguous grid instead of map

type Key = u16;

#[inline(always)]
fn parse(input: &str) -> impl Iterator<Item = ((Key, Key), (Key, Key))> + '_ {
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(from, to)| (from.split_once(',').unwrap(), to.split_once(',').unwrap()))
        .map(|((fromx, fromy), (tox, toy))| {
            (
                (fromx.parse().unwrap(), fromy.parse().unwrap()),
                (tox.parse().unwrap(), toy.parse().unwrap()),
            )
        })
}

#[inline(always)]
fn incr(x: Key, y: Key, tiles: &mut FMap<(Key, Key), u8>) {
    tiles.entry((x, y)).and_modify(|c| *c += 1).or_insert(1);
}

#[inline(always)]
fn swap_ordered(from: Key, to: Key) -> (Key, Key) {
    (from.min(to), from.max(to))
}

#[inline(always)]
fn straight_line(
    tiles: &mut FMap<(Key, Key), u8>,
    line @ ((fromx, fromy), (tox, toy)): ((Key, Key), (Key, Key)),
) {
    if fromx == tox {
        let (fromy, toy) = swap_ordered(fromy, toy);
        for y in fromy..=toy {
            incr(fromx, y, tiles);
        }
    } else if fromy == toy {
        let (fromx, tox) = swap_ordered(fromx, tox);
        for x in fromx..=tox {
            incr(x, fromy, tiles);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let tiles = parse(input).fold(fmap(50000), |mut tiles, line| {
        straight_line(&mut tiles, line);
        tiles
    });

    tiles.values().filter(|&&x| x > 1).count()
}

pub fn part2(input: &str) -> usize {
    let tiles = parse(input).fold(
        fmap(50000),
        |mut tiles, line @ ((fromx, fromy), (tox, toy))| {
            straight_line(&mut tiles, line);
            if fromx != tox && fromy != toy {
                let line @ ((fromx, fromy), (tox, toy)) = if fromx > tox {
                    ((tox, toy), (fromx, fromy))
                } else {
                    line
                };
                if fromy > toy {
                    for (x, y) in izip!(fromx..=tox, (toy..=fromy).rev()) {
                        incr(x, y, &mut tiles);
                    }
                } else {
                    for (x, y) in izip!(fromx..=tox, fromy..=toy) {
                        incr(x, y, &mut tiles);
                    }
                }
            }
            tiles
        },
    );

    tiles.values().filter(|&&x| x > 1).count()
}

#[test]
fn test() {
    let input = read_input("input5.txt").unwrap();
    assert_eq!(part1(&input), 7436);
    assert_eq!(part2(&input), 21104);
}
