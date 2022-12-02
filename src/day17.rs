use std::ops::Range;

use crate::utils::*;

type Vec2 = (isize, isize);

fn step((x, y): Vec2, (vx, vy): Vec2) -> (Vec2, Vec2) {
    ((x + vx, y + vy), (vx - vx.signum(), vy - 1))
}

fn traj((vx, vy): Vec2, tx: Range<isize>, ty: Range<isize>) -> impl Iterator<Item = (Vec2, Vec2)> {
    let pt = (0, 0);
    let dir = (vx, vy);

    successors(Some((pt, dir)), |&(pt, dir)| Some(step(pt, dir)))
        // .take_while(|&((x, y), (vx, vy))| vx >= 0 && x < tx.end && y > ty.start)
        .take_while(move |&((x, y), (vx, vy))| {
            let xok = if vx == 0 {
                tx.contains(&x)
            } else if vx > 0 {
                tx.end >= x
            } else {
                tx.start <= x
            };
            let yok = y >= ty.start;
            xok && yok
        })
}

fn hit_target((vx, vy): Vec2, tx: Range<isize>, ty: Range<isize>) -> (isize, Result<Vec2, Vec2>) {
    traj((vx, vy), tx.clone(), ty.clone()).fold((0, Err((0, 0))), |(h, end), ((x, y), v)| {
        // dbg!(((x, y), v));
        (
            isize::max(h, y),
            if end.is_err() {
                if tx.contains(&x) && ty.contains(&y) {
                    Ok((x, y))
                } else {
                    Err((x, y))
                }
            } else {
                end
            },
        )
    })
}

// fn candidates(tx: Range<isize>, ty: Range<isize>) -> impl Iterator<Item = Vec2> {
//     panic!("TODO")
// }

fn parse(input: &str) -> (Range<isize>, Range<isize>) {
    let (_, tar) = input.split_once(':').unwrap();
    dbg!(tar);
    let (x, y) = tar.split_once(',').unwrap();
    dbg!((x, y));
    let tx = {
        let (_, x) = x.split_once('=').unwrap();
        dbg!(x);
        let (a, b) = x.split_once("..").unwrap();
        dbg!((a, b));
        let (a, b) = (
            a.trim().parse::<isize>().unwrap(),
            b.trim().parse::<isize>().unwrap(),
        );
        dbg!((a, b));
        a..(b + 1)
    };
    let ty = {
        let (_, y) = y.split_once('=').unwrap();
        dbg!(y);
        let (a, b) = y.split_once("..").unwrap();
        dbg!((a, b));
        let (a, b) = (
            a.trim().parse::<isize>().unwrap(),
            b.trim().parse::<isize>().unwrap(),
        );
        dbg!((a, b));
        (a.min(b))..(a.max(b) + 1)
    };
    (tx, ty)
}

pub fn part1(input: &str) -> isize {
    let (tx, ty) = parse(input);

    dbg!("Searching:");
    // let mut dir = (1, 1);
    // loop {
    //     let (h, end) = hit_target(dir, tx.clone(), ty.clone());
    //     dbg!(dir, h, end);
    //     match end {
    //         Ok((x, y)) => dir.1 += 1,
    //         Err((x, y)) => {
    //             if x < tx.start {
    //                 dir.0 += 1;
    //             } else if x > tx.end {
    //                 if y < ty.start {
    //                     dir.1 += 1;
    //                 } else if y > ty.end {
    //                     dir.0 -= 1;
    //                 } else {
    //                     unreachable!()
    //                 }
    //             } else {
    //                 dir.1 += 1;
    //             }
    //         }
    //     };
    //     pause();
    // }
    (1..(tx.end.max(ty.end)))
        .flat_map(|i| (1..i).map(move |j| (i - j, j)))
        .map(|dir| hit_target(dir, tx.clone(), ty.clone()))
        .filter(|(_, e)| e.is_ok())
        .max_by_key(|&(h, _)| h)
        .unwrap()
        .0

    // unimplemented!()
}

pub fn part2(input: &str) -> usize {
    let (tx, ty) = parse(input);

    let mut t = vec![
        (23, -10),
        (25, -9),
        (27, -5),
        (29, -6),
        (22, -6),
        (21, -7),
        (9, 0),
        (27, -7),
        (24, -5),
        (25, -7),
        (26, -6),
        (25, -5),
        (6, 8),
        (11, -2),
        (20, -5),
        (29, -10),
        (6, 3),
        (28, -7),
        (8, 0),
        (30, -6),
        (29, -8),
        (20, -10),
        (6, 7),
        (6, 4),
        (6, 1),
        (14, -4),
        (21, -6),
        (26, -10),
        (7, -1),
        (7, 7),
        (8, -1),
        (21, -9),
        (6, 2),
        (20, -7),
        (30, -10),
        (14, -3),
        (20, -8),
        (13, -2),
        (7, 3),
        (28, -8),
        (29, -9),
        (15, -3),
        (22, -5),
        (26, -8),
        (25, -8),
        (25, -6),
        (15, -4),
        (9, -2),
        (15, -2),
        (12, -2),
        (28, -9),
        (12, -3),
        (24, -6),
        (23, -7),
        (25, -10),
        (7, 8),
        (11, -3),
        (26, -7),
        (7, 1),
        (23, -9),
        (6, 0),
        (22, -10),
        (27, -6),
        (8, 1),
        (22, -8),
        (13, -4),
        (7, 6),
        (28, -6),
        (11, -4),
        (12, -4),
        (26, -9),
        (7, 4),
        (24, -10),
        (23, -8),
        (30, -8),
        (7, 0),
        (9, -1),
        (10, -1),
        (26, -5),
        (22, -9),
        (6, 5),
        (7, 5),
        (23, -6),
        (28, -10),
        (10, -2),
        (11, -1),
        (20, -9),
        (14, -2),
        (29, -7),
        (13, -3),
        (23, -5),
        (24, -8),
        (27, -9),
        (30, -7),
        (28, -5),
        (21, -10),
        (7, 9),
        (6, 6),
        (21, -5),
        (27, -10),
        (7, 2),
        (30, -9),
        (21, -8),
        (22, -7),
        (24, -9),
        (20, -6),
        (6, 9),
        (29, -5),
        (8, -2),
        (27, -8),
        (30, -5),
        (24, -7),
    ];
    t.sort_unstable();

    let max = 2 * tx.end.max(ty.end);
    (0..=max)
        .cartesian_product(0..=max)
        .flat_map(|(i, j)| [(i, j), (-i, j), (i, -j), (-i, -j)])
        .filter(|&dir| {
            let r = hit_target(dir, tx.clone(), ty.clone());
            // println!("{:?}\t{:?}", dir, r);
            r.1.is_ok()
        })
        .sorted()
        .dedup()
        .inspect(|dir| println!("{:?}", dir))
        .count()

    // unimplemented!()
}

pub fn tests() {
    let test = "target area: x=20..30, y=-10..-5";
    // let (tx, ty) = parse(test);
    // dbg!(hit_target((7, 2), tx.clone(), ty.clone()));
    // dbg!(hit_target((6, 3), tx.clone(), ty.clone()));
    // dbg!(hit_target((6, 9), tx.clone(), ty.clone()));
    // dbg!(hit_target((9, 0), tx.clone(), ty.clone()));
    // dbg!(hit_target((17, -4), tx.clone(), ty.clone()));

    assert_eq!(part1(test), 45);
    assert_eq!(part2(test), 112);

    let input = read_input("input17.txt").unwrap();
    assert_eq!(part1(&input), 8646);
    assert_eq!(part2(&input), 0);
}

#[test]
fn test() {
    tests();
}
