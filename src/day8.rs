use std::{iter::IntoIterator, mem::swap, ops::MulAssign};

use crate::{u32set::U32Set, utils::*};

// Given a list of numbers, find the indices that are greater than all preceeding numbers
fn visible<
    T: PartialOrd + Copy + core::fmt::Display + core::fmt::Debug,
    I: Iterator<Item = T> + Clone,
>(
    mut line: I,
) -> impl Iterator<Item = usize> {
    let fst = line.next().unwrap();
    line.enumerate()
        .scan(fst, |max, (idx, num)| {
            Some(if num > *max {
                // println!("@{idx} {num:?} > {max:?}");
                *max = num;
                Some(idx + 1)
            } else {
                None
            })
        })
        .flatten()
}

fn transpose<T: Copy>(input: &[T], rows: usize, cols: usize) -> Vec<T> {
    assert!(input.len() >= rows * cols);
    let mut out = input.to_vec();
    for i in 0..rows {
        for j in 0..i {
            let (l, r) = out.split_at_mut(i * cols + j);
            swap(&mut r[0], &mut l[j * cols + i]);
        }
    }
    out
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
            .chain(bot_visible), // .inspect(|x| println!("grid[{x}] = {}", grid[*x])),
    );

    set.len() + 2 * (rows - 1) + 2 * (cols - 1)
}

pub fn part2(input: &str) -> u32 {
    // Parse into a flat vec of numbers representing the grid
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .bytes()
        .filter(|&b| b >= b'0')
        .map(|b| b - b'0')
        .collect::<Vec<_>>();
    let rows = grid.len() / cols;
    let grid_t = transpose(&grid, rows, cols);

    let mut vis = vec![1u32; grid.len()];
    calc_visibility(
        grid.chunks(cols).map(|r| r.iter().copied()),
        vis.chunks_mut(cols),
    );
    // let mut vis = vec![1u32; grid.len()];
    calc_visibility(
        grid.chunks(cols).map(|r| r.iter().rev().copied()),
        vis.chunks_mut(cols).map(|r| r.iter_mut().rev()),
    );

    let mut vis_t = vec![1u32; grid.len()];
    calc_visibility(
        grid_t.chunks(rows).map(|r| r.iter().copied()),
        vis_t.chunks_mut(rows),
    );
    // let mut vis_t = vec![1u32; grid.len()];
    calc_visibility(
        grid_t.chunks(rows).map(|r| r.iter().rev().copied()),
        vis_t.chunks_mut(rows).map(|r| r.iter_mut().rev()),
    );

    let vis_tt = transpose(&vis_t, cols, rows);
    vis.iter().zip(vis_tt).map(|(x, y)| x * y).max().unwrap()
}

fn calc_visibility<
    'k,
    T: Copy + PartialOrd,
    U: Copy + 'k + MulAssign<u32>,
    I: IntoIterator<Item = T>,
    J: IntoIterator<Item = I>,
    K: IntoIterator<Item = &'k mut U>,
>(
    grid: J,
    vis: impl IntoIterator<Item = K>,
    // vis: &mut [u32],
) where
    usize: From<T>,
{
    grid.into_iter()
        .map(IntoIterator::into_iter)
        .map(|r| {
            r.scan([0; 10], |counts, num| {
                Some({
                    let c = counts[usize::from(num)];
                    counts[..=usize::from(num)].iter_mut().for_each(|c| *c = 1);
                    counts[usize::from(num) + 1..]
                        .iter_mut()
                        .for_each(|c| *c += 1);
                    c
                })
            })
        })
        .zip(vis)
        .flat_map(|(g, v)| g.zip(v))
        .for_each(|(num, out)| *out *= num);
}

#[test]
fn test() {
    let input = read_input("input8.txt").unwrap();
    assert_eq!(part1(&input), 1859);
    let test = "30373
25512
65332
33549
35390";
    assert_eq!(part2(&input), 332640);
}
