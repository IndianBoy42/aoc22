use crate::{u32set::U32Set, utils::*};

const SEGMENTS: &[&[u8]] = &[
    &[b'a', b'b', b'c', b'e', b'f', b'g'],       // 0
    &[b'c', b'f'],                               // 1
    &[b'a', b'c', b'd', b'e', b'g'],             // 2
    &[b'a', b'c', b'd', b'f', b'g'],             // 3
    &[b'b', b'c', b'd', b'f'],                   // 4
    &[b'a', b'b', b'd', b'f', b'g'],             // 5
    &[b'a', b'b', b'd', b'e', b'f', b'g'],       // 6
    &[b'a', b'c', b'f'],                         // 7
    &[b'a', b'b', b'c', b'd', b'e', b'f', b'g'], // 8
    &[b'a', b'b', b'c', b'd', b'f', b'g'],       // 9
];

const UNIQUE: [(u8, u8); 4] = [(1, 2), (4, 4), (7, 3), (8, 7)];
const UNIQUE_LENS: [usize; 4] = [2, 4, 3, 7];

pub fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| (left.split(' '), right.split(' ')));

    lines
        .map(|(_, right)| right)
        .map(|output| {
            output
                .filter_map(|num| UNIQUE.iter().find(|&&(_, segs)| num.len() == segs as usize))
                .count()
        })
        .sum()
}

pub fn part2_(input: &str) -> usize {
    let parse = |x: &str| {
        x.trim()
            .split(' ')
            .map(|x| x.bytes().collect::<ArrayVec<_, 8>>())
            .update(|x| x.sort_unstable())
            .collect::<ArrayVec<_, 10>>()
    };
    // .update(|(left, _)| left.retain(|seq| UNIQUE_LENS.iter().all(|&l| seq.len() != l)));

    input
        .par_lines()
        // .map(|x| dbg!(x))
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| (parse(left), parse(right)))
        .map(|(left, right)| {
            b"abcdefg"
                .iter()
                .permutations(7)
                .find(|order| {
                    left.iter().all(|seq| {
                        let code = seq
                            .iter()
                            .map(|&c| c - b'a')
                            .map(|i| *order[i as usize])
                            .sorted_unstable()
                            .collect::<ArrayVec<_, 8>>();
                        SEGMENTS.iter().any(|&m| m == code.as_slice())
                        //.map(|x| dbg!(x))
                    })
                })
                .map(|order| {
                    right
                        .iter()
                        .map(|seq| {
                            let code = seq
                                .iter()
                                .map(|&c| c - b'a')
                                .map(|i| *order[i as usize])
                                .sorted_unstable()
                                .collect::<ArrayVec<_, 8>>();
                            SEGMENTS
                                .iter()
                                .position(|&m| m == code.as_slice())
                                // .map(|c| (b'0' + c ) )
                                .expect("Must be valid")
                        })
                        .collect::<ArrayVec<_, 4>>()
                })
                .expect("Must have a valid order")
        })
        .map(|digits| {
            digits.into_iter().fold(0, |acc, x| acc * 10 + x)
            //     .collect::<String>()
            //     .parse::<usize>()
            //     .unwrap()
        })
        .sum::<usize>()
}

pub fn part2_bitset2(input: &str) -> usize {
    let parse = |x: &str| {
        x.trim()
            .split(' ')
            .map(|x| x.bytes().map(|x| x - b'a').collect::<U32Set>())
            .collect::<ArrayVec<_, 10>>()
    };

    let refmap = SEGMENTS.iter().flat_map(|a| a.iter()).copied().counts();
    let score_to_digit = SEGMENTS
        .iter()
        .enumerate()
        .map(|(i, segment)| (segment.iter().map(|c| refmap[c] as u8).sum(), i as _))
        .collect::<FMap<u8, u8>>();
    // .update(|(left, _)| left.retain(|seq| UNIQUE_LENS.iter().all(|&l| seq.len() != l)));
    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| (parse(left), parse(right)))
        .map(|(left, right)| {
            let map = left.iter().flat_map(|a| a.ones()).counts();
            let seq_to_digit = left
                .iter()
                .map(|segment| (segment, segment.ones().map(|c| map[&c] as u8).sum()))
                .map(|(seg, score)| (seg, score_to_digit[&score]))
                .collect::<ArrayVec<_, 10>>();

            right
                .iter()
                .map(|code| {
                    *seq_to_digit
                        .iter()
                        .find_map(|(seq, digit)| (seq == &code).then_some(digit))
                        .unwrap() as usize
                })
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum::<usize>()
}
pub fn part2(input: &str) -> usize {
    let parse = |x: &str| {
        x.trim()
            .split(' ')
            // TODO: represent sequence as bitset to remove the need for sorting
            .map(|x| x.bytes().collect::<ArrayVec<_, 8>>())
            .update(|x| x.sort_unstable())
            .collect::<ArrayVec<_, 10>>()
    };

    let refmap = SEGMENTS.iter().flat_map(|a| a.iter()).copied().counts();
    let score_to_digit = SEGMENTS
        .iter()
        .enumerate()
        .map(|(i, segment)| (segment.iter().map(|c| refmap[c] as u8).sum(), i as _))
        .collect::<FMap<u8, u8>>();
    // .update(|(left, _)| left.retain(|seq| UNIQUE_LENS.iter().all(|&l| seq.len() != l)));
    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| (parse(left), parse(right)))
        .map(|(left, right)| {
            let map = left.iter().flat_map(|a| a.iter()).copied().counts();
            let seq_to_digit = left
                .iter()
                .map(|segment| (segment, segment.iter().map(|c| map[c] as u8).sum()))
                .map(|(seg, score)| (seg, score_to_digit[&score]))
                .collect::<ArrayVec<_, 10>>();

            right
                .iter()
                .map(|code| {
                    *seq_to_digit
                        .iter()
                        .find_map(|(seq, digit)| (seq == &code).then_some(digit))
                        .unwrap() as usize
                })
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum::<usize>()
}

#[test]
fn test() {
    let input = read_input("input8.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part2(&input), 1_010_472);
    assert_eq!(part1(&input), 330);
}

pub fn part2_bitset(input: &str) -> usize {
    let parse = |x: &str| {
        x.trim()
            .split(' ')
            .map(|x| x.bytes().map(|x| x - b'a').collect::<U32Set>())
            .collect::<ArrayVec<_, 10>>()
    };
    // .update(|(left, _)| left.retain(|seq| UNIQUE_LENS.iter().all(|&l| seq.len() != l)));

    let segments = SEGMENTS
        .iter()
        .map(|&seq| seq.iter().map(|x| x - b'a').collect::<U32Set>())
        .collect::<ArrayVec<_, 10>>();

    input
        // .par_lines()
        .lines()
        // .map(|x| dbg!(x))
        .map(|line| line.split_once('|').unwrap())
        .map(|(left, right)| (parse(left), parse(right)))
        .map(|(left, right)| {
            b"abcdefg"
                .iter()
                .permutations(7)
                .find(|order| {
                    left.iter().all(|seq| {
                        let code = seq
                            .ones()
                            .map(|i| *order[i as usize] - b'a')
                            .collect::<U32Set>();
                        segments.iter().any(|&m| m == code)
                        //.map(|x| dbg!(x))
                    })
                })
                .map(|order| {
                    right
                        .iter()
                        .map(|seq| {
                            let code = seq
                                .ones()
                                .map(|i| *order[i as usize] - b'a')
                                .collect::<U32Set>();
                            segments
                                .iter()
                                .position(|&m| m == code)
                                // .map(|c| (b'0' + c ) )
                                .expect("Must be valid")
                        })
                        .collect::<ArrayVec<_, 4>>()
                })
                .expect("Must have a valid order")
        })
        .map(|digits| {
            digits.into_iter().fold(0, |acc, x| acc * 10 + x)
            //     .collect::<String>()
            //     .parse::<usize>()
            //     .unwrap()
        })
        .sum::<usize>()
}
