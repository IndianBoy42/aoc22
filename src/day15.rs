use std::{hash::Hash, iter};

use crate::{
    grid::{adj_neighbours_if, Grid2D},
    searcher::DijSearcher,
    utils::*,
};

fn parse(input: &str) -> Grid2D<u32> {
    let digits = input.lines().enumerate().flat_map(|(row, line)| {
        line.bytes()
            .map(|b| (b - b'0') as _)
            .enumerate()
            .map(move |(col, b)| ((row, col), b))
    });
    digits.collect()
}

#[derive(Debug, Copy, Clone)]
struct Tile((u32, u32), u32);

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

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// struct TileAStar((u32, u32), u32);
//
// impl TileAStar {
//     fn dist(self) -> usize {
//         let TileAStar((x, y), _) = self;
//         (x + y) as _
//     }
// }
//
// impl PartialOrd for TileAStar {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.dist().partial_cmp(&other.dist())
//     }
// }
// impl Ord for TileAStar {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.dist().cmp(&other.dist())
//     }
// }

pub fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let grid_shape = grid.shape();
    let (gx, gy) = grid_shape;
    let (gx, gy) = (gx as u32, gy as u32);

    let risk = |(x, y)| *grid.get(&(x as _, y as _)).unwrap();
    let tile = |n @ (x, y), v| Tile(n, v + risk(n));

    let cost = DijSearcher::<_, FSet<_>, _>::new(tile((0, 0), 0), |&t: &Tile| {
        let Tile(node, v) = t;
        let neighbours = adj_neighbours_if(node, |&(x, y)| grid.check((x as _, y as _)));
        neighbours
            .into_iter()
            .map(|next| tile(next, v))
            .collect::<ArrayVec<Tile, 4>>()
    })
    .check()
    // .map(|tile| dbg!(tile))
    .find(|&Tile((r, c), v)| (r + 1, c + 1) == (gx, gy));

    cost.unwrap().1 - grid.get(&(0, 0)).unwrap()
}

pub fn part2(input: &str) -> u32 {
    let grid = parse(input);
    let grid_shape = grid.shape();
    let (gx, gy) = grid_shape;
    let (gx, gy) = (gx as u32, gy as u32);
    let upsize = 5;

    let risk = |(x, y)| {
        let (ix, iy) = (x / gx, y / gy);
        let (x, y) = (x % gx, y % gy);
        ((*grid.get(&(x as _, y as _)).unwrap() + ix as u32 + iy as u32 - 1) % 9) + 1
    };
    let tile = |n, v| Tile(n, v + risk(n));

    let cost = DijSearcher::<_, FSet<_>, _>::new(tile((0, 0), 0), |&t: &Tile| {
        let Tile(node, v) = t;
        let neighbours = adj_neighbours_if(node, |&(x, y)| x < gx * upsize && y < gy * upsize);
        neighbours
            .into_iter()
            .map(|next| tile(next, v))
            .collect::<ArrayVec<Tile, 4>>()
    })
    .check()
    // .map(|tile| dbg!(tile))
    .find(|&Tile((r, c), v)| (r + 1, c + 1) == (gx * (upsize), gy * (upsize)));

    cost.unwrap().1 - grid.get(&(0, 0)).unwrap()
}

#[test]
fn test() {
    let input = read_input("input15.txt").unwrap();
    assert_eq!(part1(&input), 755);
    assert_eq!(part2(&input), 3016);
}
