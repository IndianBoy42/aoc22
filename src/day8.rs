use crate::{u32set::U32Set, utils::*};

// Given a list of numbers, find the indices that are greater than all preceeding numbers
fn visible<
    T: PartialOrd + Copy + core::fmt::Display + core::fmt::Debug,
    I: Iterator<Item = T> + Clone,
>(
    mut line: I,
) -> impl Iterator<Item = usize> {
    {
        let line = line.clone().join("");
        println!("{line}");
    }
    let fst = line.next().unwrap();
    line.enumerate()
        .scan(fst, |max, (idx, num)| {
            Some(if num > *max {
                println!("@{idx} {num:?} > {max:?}");
                *max = num;
                Some(idx + 1)
            } else {
                None
            })
        })
        .flatten()
}

pub fn part1(input: &str) -> usize {
    // Example input:
    // 30373
    // 25512
    // 65332
    // 33549
    // 35390
    // Parse into a flat vec of numbers representing the grid
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .bytes()
        .filter(|&b| b >= b'0')
        .map(|b| b - b'0')
        .collect::<Vec<_>>();
    let rows = grid.len() / cols;

    let left_visible = (1..rows - 1)
        .map(|nrow| (nrow, grid[nrow * cols..][0..cols - 1].iter().copied()))
        .flat_map(|(nrow, r)| visible(r).map(move |i| nrow * cols + i));

    let right_visible = (1..rows - 1)
        .map(|nrow| (nrow, grid[nrow * cols..][1..cols].iter().rev().copied()))
        .flat_map(|(nrow, r)| visible(r).map(move |i| (nrow + 1) * cols - i - 1));

    let top_visible = (1..cols - 1)
        .map(|ncol| {
            (
                ncol,
                grid[ncol..][0..cols * (rows - 1)]
                    .iter()
                    .copied()
                    .step_by(cols),
            )
        })
        .flat_map(|(ncol, c)| visible(c).map(move |i| i * cols + ncol));

    let bot_visible = (1..cols - 1)
        .map(|ncol| {
            (
                ncol,
                grid[ncol + cols..cols * rows]
                    .iter()
                    .copied()
                    .step_by(cols)
                    .rev(),
            )
        })
        .flat_map(|(ncol, c)| visible(c).map(move |i| (rows - i - 1) * cols + ncol));

    let mut set = fset::<_>(1000);
    set.extend(
        left_visible
            .chain(right_visible)
            .chain(top_visible)
            .chain(bot_visible)
            .inspect(|x| println!("grid[{x}] = {}", grid[*x])),
    );
    set.len() + 2 * (rows - 1) + 2 * (cols - 1)
}

pub fn part2(input: &str) -> usize {
    // Parse into a flat vec of numbers representing the grid
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .bytes()
        .filter(|&b| b >= b'0')
        .map(|b| b - b'0')
        .collect::<Vec<_>>();
    let rows = grid.len() / cols;
}

#[test]
fn test() {
    let input = read_input("input8.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    let tst = "30373
25512
65332
33549
35390";
    assert_eq!(part1(&input), 1859);
    assert_eq!(part2(&input), 0);
}
