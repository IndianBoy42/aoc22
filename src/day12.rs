use std::io::stdin;
use std::{hash::Hash, iter};

use arrayvec::ArrayVec;

use crate::{
    grid::{adj_neighbours_if, Grid2D},
    searcher::{BFSearcher, DijSearcher},
    utils::*,
};

#[derive(Debug, Copy, Clone)]
struct Tile((usize, usize), usize);

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Tile {}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

pub fn part1(input: &str) -> usize {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| line.bytes().position(|b| b == b'S').map(|col| (row, col)))
        .unwrap();
    let end = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| line.bytes().position(|b| b == b'E').map(|col| (row, col)))
        .unwrap();
    let heights = input.lines().flat_map(|line| {
        line.bytes().map(|b| match b {
            // a to z are 0-25
            b'a'..=b'z' => b,
            b'S' => b'a',
            b'E' => b'z',
            _ => unreachable!(),
        })
    });
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let grid = Grid2D::from_iter_w_shape(shape, heights);

    // let start = start.0 * shape.1 + start.1;
    // let end = end.0 * shape.1 + end.1;
    // let grid = heights.collect::<Vec<_>>();

    BFSearcher::<_, FSet<_>, _>::new(Tile(end, 0), |&Tile(pos, v): &Tile| {
        let curr_height = grid[pos];
        adj_neighbours_if(pos, |&neighbor| {
            grid.check(neighbor) && grid[neighbor] >= (curr_height - 1)
        })
        .into_iter()
        .map(move |pos| Tile(pos, v + 1))
    })
    .check()
    .find_map(|Tile(pos, v)| (pos == start).as_some(v))
    .unwrap()
}

pub fn part2(input: &str) -> usize {
    let start = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| line.bytes().position(|b| b == b'S').map(|col| (row, col)))
        .unwrap();
    let end = input
        .lines()
        .enumerate()
        .find_map(|(row, line)| line.bytes().position(|b| b == b'E').map(|col| (row, col)))
        .unwrap();
    let heights = input.lines().flat_map(|line| {
        line.bytes().map(|b| match b {
            // a to z are 0-25
            b'a'..=b'z' => b,
            b'S' => b'a',
            b'E' => b'z',
            _ => unreachable!(),
        })
    });
    let shape = (input.lines().count(), input.lines().next().unwrap().len());
    let grid = Grid2D::from_iter_w_shape(shape, heights);

    // let start = start.0 * shape.1 + start.1;
    // let end = end.0 * shape.1 + end.1;
    // let grid = heights.collect::<Vec<_>>();

    BFSearcher::<_, FSet<_>, _>::new(Tile(end, 0), |&Tile(pos, v): &Tile| {
        let curr_height = grid[pos];
        adj_neighbours_if(pos, |&neighbor| {
            grid.check(neighbor) && grid[neighbor] >= (curr_height - 1)
        })
        .into_iter()
        .map(move |pos| Tile(pos, v + 1))
    })
    .check()
    .find_map(|Tile(pos, v)| (grid[pos] == b'a').as_some(v))
    .unwrap()
}

#[test]
fn test() {
    let input = read_input("input12.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 391);
    assert_eq!(part2(&input), 386);
}
