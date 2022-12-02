use std::{ops::ControlFlow, str::from_utf8};

use crate::utils::*;

const MARK: u8 = !0;

fn check_board(board: &[u8]) -> bool {
    if board
        .array_chunks()
        .any(|row: &[_; 5]| row.iter().all(|&x| x == MARK))
    {
        return true;
    }
    // if (0..5).any(|i| (i..25).step_by(5).map(|i| board[i]).all(|x| x == 0)) {
    // if (0..5).any(|i| board[i..].iter().step_by(5).all(|&x| x == 0)) {
    if (0..5).any(|i| board[i..].iter().step_by(5).all(|&x| x == MARK)) {
        return true;
    }

    // // Diagonal 1
    // if (0..25).step_by(6).map(|i| board[i]).all(|x| x == 0) {
    //     return true;
    // }
    // // Diagonal 2
    // if (4..25).step_by(4).map(|i| board[i]).all(|x| x == 0) {
    //     return true;
    // }

    false
}

fn board_score(winner: impl IntoIterator<Item = u8>, called: usize) -> usize {
    winner
        .into_iter()
        .filter(|&x| x != MARK)
        .map(|x| x as usize)
        .sum::<usize>()
        * called
}

pub fn play_bingo(
    drawn: impl Iterator<Item = u8>,
    mut boards: Vec<Vec<u8>>,
    mut f: impl FnMut(Vec<u8>, u8) -> bool,
) {
    for number in drawn {
        let br = boards
            .drain_filter(|board| {
                let count = board
                    .iter_mut()
                    .filter(|x| **x == number)
                    .update(|x| **x = MARK)
                    .count();
                (count > 0) && check_board(board)
            })
            .any(|board| f(board, number));

        if br {
            break;
        }
    }
}

pub fn drawn(input: &str) -> impl Iterator<Item = u8> + '_ {
    input.split(',').map(str::parse).map(Result::unwrap)
}
pub fn boards<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<u8>> {
    input
        .map(|board| {
            board
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec()
}

pub fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let drawn = drawn(blocks.next().unwrap());
    let boards = boards(blocks);

    let mut winner = None;
    let mut called = 0;
    play_bingo(drawn, boards, |board, number| {
        winner = Some(board);
        called = number;
        true
    });

    board_score(winner.expect("Must have winner"), called as usize)
}

pub fn part2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let drawn = drawn(blocks.next().unwrap());
    let boards = boards(blocks);

    let mut winner = None;
    let mut called = 0;
    play_bingo(drawn, boards, |board, number| {
        winner = Some(board);
        called = number;
        false
    });

    board_score(winner.expect("Must have winner"), called as usize)
}

#[test]
fn test() {
    let input = read_input("input4.txt").unwrap();
    assert_eq!(part1(&input), 60368);
    assert_eq!(part2(&input), 17435);
}
