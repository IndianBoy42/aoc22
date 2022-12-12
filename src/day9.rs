use crate::{
    grid::{self, adj_neighbours, adj_neighbours_if, Grid2D},
    searcher::{BFSearcher, DFSearcher},
    utils::*,
};
use lazysort::SortedBy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'U' => Self::Up,
            b'D' => Self::Down,
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn apply((head, tail): ((i32, i32), (i32, i32)), dir: Direction) -> ((i32, i32), (i32, i32)) {
    let head = incr(dir, head);
    let tail = follow(head, tail);

    (head, tail)
}

fn incr(dir: Direction, head: (i32, i32)) -> (i32, i32) {
    match dir {
        Direction::Up => (head.0, head.1 - 1),
        Direction::Down => (head.0, head.1 + 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
    }
}

fn follow(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    // 25-1 possible positions of the head relative to the tail: (-2, -1, 0, 1, 2) x (-2, -1, 0, 1, 2)
    // The tail moves to be within 1 step of the head
    match (head.0 - tail.0, head.1 - tail.1) {
        (0 | 1 | -1, 0 | 1 | -1) => tail,
        // (0, y) => (tail.0, tail.1 + y.signum()),
        // (x, 0) => (tail.0 + x.signum(), tail.1),
        (x, y) => (tail.0 + x.signum(), tail.1 + y.signum()),
    }
}

const START: (i32, i32) = (0, 0);
pub fn part1(input: &str) -> usize {
    let visited = parse(input)
        .scan([START, START], |[head, tail], d| {
            *head = incr(d, *head);
            *tail = follow(*head, *tail);
            Some(*tail)
        })
        .collect::<FSet<_>>();

    visited.len()
}

pub fn part2(input: &str) -> usize {
    let visited = parse(input)
        .scan([START; 10], |snake, d| {
            {
                let (head, tail) = snake.split_first_mut().unwrap();
                *head = incr(d, *head);
                tail.iter_mut().fold(*head, |prev, next| {
                    *next = follow(prev, *next);
                    *next
                });
            }
            Some(*snake.last().unwrap())
        })
        .collect::<FSet<_>>();

    visited.len()
}

fn parse(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .lines()
        .map(|line| {
            (
                line.as_bytes()[0].into(),
                line.split_at(2).1.parse().unwrap(),
            )
        })
        .flat_map(|(d, n)| (0..n).map(move |_| d))
}

#[test]
fn test() {
    let input = read_input("input9.txt").unwrap();

    assert_eq!(part1(&input), 5930);

    let test = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(part2(&input), 0);
}
