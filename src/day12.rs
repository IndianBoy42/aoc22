use std::io::stdin;

use crate::{searcher::BFSearcher, utils::*};
fn push<'a, F>(
    q: &mut Vec<(&'a str, &'a str)>,
    adj: &HashMap<&'a str, FSet<&'a str>>,
    from: &'a str,
    filt: F,
) where
    F: for<'r> FnMut(&'r (&'a str, &'a str)) -> bool,
{
    q.extend(
        adj.get(from)
            .unwrap()
            .iter()
            .map(|&node| (from, node))
            .filter(filt),
    );
}

fn solver<'a, T>(
    visited: &[&str],
    adj: &HashMap<&'a str, FSet<&'a str>>,
    from: &'a str,
    filt: fn(&[&str], &str, T) -> Option<T>,
    state: T,
) -> usize
where
    T: Copy,
{
    if from == "end" {
        println!("{}", visited.iter().join(","));
        return 1;
    }
    // dbg!(visited);
    let mut visited = visited.to_vec();
    visited.push(from);
    adj.get(from)
        .unwrap()
        .iter()
        .copied()
        .filter_map(|node| filt(&visited, node, state).map(|t| (node, t)))
        .map(|(node, state)| solver(&visited, adj, node, filt, state))
        .sum()
}

fn all_lower(node: &str) -> bool {
    node.bytes().all(|c| c.is_ascii_lowercase())
}

pub fn part1(input: &str) -> usize {
    let adj = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .into_grouping_map()
        .collect::<FSet<_>>();

    solver(
        &[],
        &adj,
        "start",
        |visited, node, _| {
            (if all_lower(node) {
                visited.iter().all(|&x| x != node)
            } else {
                true
            })
            .then_some(())
        },
        (),
    )
}

pub fn part2(input: &str) -> usize {
    let mut adj = input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .filter(|&(_, r)| r != "start")
        .into_grouping_map()
        .collect::<FSet<_>>();
    adj.remove("end");

    solver(
        &[],
        &adj,
        "start",
        |visited, node, already_repeated| {
            if all_lower(node) {
                let any = visited.iter().all(|&x| x != node);
                if any {
                    Some(already_repeated)
                } else if already_repeated {
                    None
                } else {
                    Some(true)
                }
            } else {
                Some(already_repeated)
            }
        },
        false,
    )
}

#[test]
fn test() {
    let input = read_input("input12.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part2(&input), 133_621);
    assert_eq!(part1(&input), 4378);
}
