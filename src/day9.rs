use crate::{
    grid::{self, adj_neighbours, adj_neighbours_if, Grid2D},
    searcher::{BFSearcher, DFSearcher},
    utils::*,
};
use lazysort::SortedBy;

type MapGrid2D<T> = FMap<(u8, u8), T>;
type Grid = Grid2D<u8>;
// type Grid = MapGrid2D<u8>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.bytes()
                .enumerate()
                .map(move |(c, cha)| ((r as _, c as _), cha - b'0'))
        })
        // .collect::<FMap<(usize, usize), _>>();
        .collect::<Grid>()
}

fn minima(map: &Grid) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    map.iter()
        .map(|((r, c), &num)| ((r, c), num))
        .filter(|&((r, c), num)| {
            let a = map.get(&(r + 1, c)).copied().unwrap_or(10);
            let b = r
                .checked_sub(1)
                .and_then(|r| map.get(&(r, c)))
                .copied()
                .unwrap_or(10);
            let d = c
                .checked_sub(1)
                .and_then(|c| map.get(&(r, c)))
                .copied()
                .unwrap_or(10);
            let c = map.get(&(r, c + 1)).copied().unwrap_or(10);

            (num < a) && (num < b) && (num < c) && (num < d)
        })
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);

    minima(&map).map(|(_, num)| 1 + num as usize).sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);

    if false {
        let basin_sizes = map
            .iter()
            .map(|((r, c), &num)| ((r, c), num))
            .filter(|&(_, num)| num != 9)
            .scan(
                DFSearcher::<(u8, u8), FSet<(u8, u8)>, _>::new_empty(|p: &(u8, u8)| {
                    adj_neighbours_if(*p, |&(r, c)| {
                        map.get(&(r as _, c as _)).map_or(false, |&h| h != 9)
                    })
                }),
                |st, ((r, c), num)| {
                    if st.push((r, c)) {
                        let mut i = 0;
                        for _ in st.by_ref() {
                            i += 1;
                        }
                        Some(i)
                    } else {
                        Some(0)
                    }
                },
            )
            .filter(|&x| x > 0);

        if true {
            let mut points: Vec<_> = basin_sizes.collect();
            points.sort_unstable_by(|a, b| b.cmp(a));
            points[..3].iter().product()
        } else {
            lazysort::SortedBy::sorted_by(basin_sizes, |a, b| b.cmp(a))
                .take(3)
                .product()
        }
    } else {
        // TODO: watershed algorithm, no minima needed
        let points = minima(&map).collect_vec();

        let basin_sizes = points.into_par_iter().map(|((row, col), _)| {
            DFSearcher::<(u8, u8), FSet<(u8, u8)>, _>::with_capacity(
                map.len(),
                (row as u8, col as u8),
                |p: &(u8, u8)| {
                    adj_neighbours_if(*p, |&(r, c)| {
                        map.get(&(r as _, c as _)).map_or(false, |&h| h != 9)
                    })
                },
            )
            .check()
            .count()
        });

        if true {
            let mut points: Vec<_> = basin_sizes.collect();
            points.sort_unstable_by(|a, b| b.cmp(a));
            points[..3].iter().product()
        } else {
            // lazysort::SortedBy::sorted_by(basin_sizes, |a, b| b.cmp(a))
            //     .take(3)
            //     .product()
            unimplemented!()
        }
    }
}

#[test]
fn test() {
    let input = read_input("input9.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 528);
    assert_eq!(part2(&input), 920_448);
}
