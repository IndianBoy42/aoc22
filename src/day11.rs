use crate::{
    grid::{all_neighbours, all_neighbours_if, Grid2D},
    searcher::{BFSearcher, DFSearcher},
    utils::*,
};

fn sim(octopi: &mut Grid2D<u8>) -> usize {
    // for (_, ele) in octopi.iter_mut::<u8>() {
    //     *ele += 1;
    // }

    let seeds = octopi
        .iter_mut()
        .update(|(_, lvl)| **lvl += 1)
        .filter(|(_, lvl)| **lvl > 9)
        .map(|(p, _)| p)
        .collect_vec();
    DFSearcher::<_, FSet<_>, _>::new_all(seeds, |&p: &(usize, usize)| {
        all_neighbours_if(p, |p| {
            if let Some(oc) = octopi.get_mut(p) {
                // *oc += 1;
                *oc += 1;
                *oc > 9
            } else {
                false
            }
        })
    })
    .check()
    .collect_vec()
    .into_iter()
    .inspect(|p| *octopi.get_mut(p).unwrap() = 0)
    .count()
}

pub fn part1(input: &str) -> usize {
    let octopi = input.lines().flat_map(str::bytes).map(|c| c - b'0');
    let mut octopi = Grid2D::from_iter_w_shape((10, 10), octopi);

    (0..100).map(|_| sim(&mut octopi)).sum()
}

pub fn part2(input: &str) -> usize {
    let octopi = input.lines().flat_map(str::bytes).map(|c| c - b'0');
    let mut octopi = Grid2D::from_iter_w_shape((10, 10), octopi);

    #[allow(clippy::maybe_infinite_iter)]
    (0..)
        .map(|_| sim(&mut octopi))
        .position(|c| c == 100)
        .unwrap()
        + 1
}

#[test]
fn test() {
    let input = read_input("input11.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 1620);
    assert_eq!(part2(&input), 371);
}
