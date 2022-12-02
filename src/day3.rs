use std::{ops::ControlFlow, str::from_utf8};

use crate::utils::*;

pub fn part1(input: &str) -> usize {
    // let lines = input.lines().count();
    let n = input.lines().clone().next().unwrap().len();
    // let n = input.split_once('\n').unwrap().0.len();
    let counts = input.lines().fold(vec![0; n], |mut acc, line| {
        line.bytes().zip(&mut acc).for_each(|(byte, ones)| {
            if byte == b'0' {
                *ones -= 1;
            } else {
                *ones += 1;
            }
        });
        acc
    });

    // let (gamma, eps) = counts
    // .into_iter()
    // .fold((0, 0), |(gamma, eps), (zeros, ones)| {
    // let bit = (zeros < ones) as usize;
    // (gamma << 1 | bit, eps << 1 | (1 - bit))
    // });
    let gamma = counts
        .into_iter()
        .map(|ones| (ones > 0) as usize)
        .fold(0, |gamma, bit| gamma << 1 | bit);
    let eps = !gamma & !(!0 << n);

    eps * gamma
}
pub fn part12(input: &str) -> usize {
    let n = input.lines().clone().next().unwrap().len();
    // let n = input.split_once('\n').unwrap().0.len();
    let counts = input.lines().fold(vec![(0, 0); n], |mut acc, line| {
        line.bytes()
            .zip(&mut acc)
            .for_each(|(byte, (zeros, ones))| {
                if byte == b'0' {
                    *zeros += 1;
                } else {
                    *ones += 1;
                }
            });
        acc
    });

    // let (gamma, eps) = counts
    // .into_iter()
    // .fold((0, 0), |(gamma, eps), (zeros, ones)| {
    // let bit = (zeros < ones) as usize;
    // (gamma << 1 | bit, eps << 1 | (1 - bit))
    // });
    let gamma = counts.into_iter().fold(0, |gamma, (zeros, ones)| {
        let bit = (zeros < ones) as usize;
        gamma << 1 | bit
    });
    let eps = !gamma & !(!0 << n);

    eps * gamma
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines().map(str::as_bytes).collect_vec();

    let ox = lines.clone();
    let ox = to_number(filter(ox, |s, gamma| s == gamma).iter().copied());

    let co = lines;
    let co = to_number(filter(co, |s, gamma| s != gamma).iter().copied());

    ox * co
}

fn to_number(ox: impl IntoIterator<Item = u8>) -> usize {
    ox.into_iter()
        .fold(0, |acc, bit| acc << 1 | (bit == b'1') as usize)
}

fn filter<F>(mut ox: Vec<&[u8]>, f: F) -> &[u8]
where
    F: Fn(u8, u8) -> bool,
{
    for i in 0.. {
        // Count most common bit
        ox = filter_once(i, ox, &f);
        if ox.len() == 1 {
            break;
        } else if ox.is_empty() {
            unreachable!();
        }
    }
    ox.first().unwrap()
}

fn filter_once<F>(i: usize, ox: Vec<&[u8]>, f: F) -> Vec<&[u8]>
where
    F: Fn(u8, u8) -> bool,
{
    // let ones = ox.iter().map(|x| x[i]).filter(|&x| x == b'1').count();
    let ones = ox
        .iter()
        .map(|x| unsafe { *x.get_unchecked(i) })
        .filter(|&x| x == b'1')
        .count();
    let most = (ones >= (ox.len() - ones)) as u8 + b'0';
    ox.into_iter().filter(|s| f(s[i], most)).collect_vec()
    // ox.retain(|s| f(s[i], most)); // So much slower!?
    // ox
}

#[test]
fn test() {
    let input = read_input("input3.txt").unwrap();
    assert_eq!(part1(&input), 2_498_354);
    assert_eq!(part2(&input), 3_277_956);
}
