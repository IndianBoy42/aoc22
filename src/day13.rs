use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<FSet<_>>();

    folds
        .lines()
        .map(|line| {
            line.split_once("along ")
                .unwrap()
                .1
                .split_once('=')
                .unwrap()
        })
        .map(|(left, right)| (left, right.parse::<usize>().unwrap()))
        .take(1)
        .fold(points, |mut points, (dir, pos)| {
            let flipped = points
                .drain_filter(|&(x, y)| {
                    if dir == "x" {
                        // Left, x changes
                        x > pos
                    } else {
                        // Up, y changes
                        y > pos
                    }
                })
                .collect_vec()
                .into_iter();
            points.extend(flipped.map(|(x, y)| {
                if dir == "y" {
                    (x, 2 * pos - y)
                } else {
                    (2 * pos - x, y)
                }
            }));
            points
        })
        .len()
}

pub fn part2(input: &str) -> String {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<FSet<_>>();

    let points = folds
        .lines()
        .map(|line| {
            line.split_once("along ")
                .unwrap()
                .1
                .split_once('=')
                .unwrap()
        })
        .map(|(left, right)| (left, right.parse::<usize>().unwrap()))
        .fold(points, |mut points, (dir, pos)| {
            let flipped = points
                .drain_filter(|&(x, y)| {
                    if dir == "x" {
                        // Left, x changes
                        x > pos
                    } else {
                        // Up, y changes
                        y > pos
                    }
                })
                .collect_vec()
                .into_iter();
            points.extend(flipped.map(|(x, y)| {
                if dir == "y" {
                    (x, 2 * pos - y)
                } else {
                    (2 * pos - x, y)
                }
            }));
            points
        });

    let (xmin, xmax) = match points.iter().copied().map(|(x, _)| x).minmax() {
        itertools::MinMaxResult::NoElements | itertools::MinMaxResult::OneElement(_) => {
            unreachable!()
        }
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    let (ymin, ymax) = match points.iter().copied().map(|(_, y)| y).minmax() {
        itertools::MinMaxResult::NoElements | itertools::MinMaxResult::OneElement(_) => {
            unreachable!()
        }
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    let mut grid = vec![vec![false; xmax - xmin + 1]; ymax - ymin + 1];
    for (x, y) in points {
        grid[y - ymin][x - xmin] = true;
    }
    for row in grid {
        for pt in row {
            print!("{}", if pt { '·' } else { ' ' });
        }
        println!();
    }

    "RZKZLPGH".into()
}

#[test]
fn test() {
    let input = read_input("input13.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 765);
    assert_eq!(part2(&input), "RZKZLPGH");
}
