use crate::utils::*;

fn parse(
    input: &str,
) -> Option<(
    Vec<Vec<char>>,
    impl Iterator<Item = (usize, usize, usize)> + '_,
)> {
    let (crates, moves) = input.split_once("\n\n")?;
    let n: usize = crates
        .lines()
        .last()?
        .split_whitespace()
        .last()?
        .parse()
        .ok()?;
    let crates = crates
        .lines()
        .map(|line| {
            // get characters #1, #5, #9, #13, ..., 4n-3
            line.chars().skip(1).step_by(4).collect::<Vec<_>>()
        })
        .rev()
        .skip(1)
        .collect::<Vec<_>>();
    let crates = (0..n)
        .map(|i| {
            crates
                .iter()
                .map(|row| row[i])
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let moves = moves.lines().filter_map(|line| {
        let mut words = line.split_whitespace();
        Some((
            words.nth(1)?.parse().ok()?,
            words.nth(1)?.parse::<usize>().ok()? - 1,
            words.nth(1)?.parse::<usize>().ok()? - 1,
        ))
    });
    Some((crates, moves))
}
pub fn part1(input: &str) -> String {
    if let Some(res) = try {
        let (crates, moves) = parse(input)?;

        let crates = moves.fold(
            crates,
            |mut crates, (num, from, to): (usize, usize, usize)| {
                // Move `num` crates from  crates[from] to crates[to], one by one, starting from the top
                let (cfrom, cto) = from_to(from, to, &mut crates);
                // Get last `num` elements of `cfrom`, reverse in place, and append to `cto`
                cto.extend(cfrom.drain(cfrom.len() - num..).rev());
                // Print `crates`
                for row in &crates {
                    println!("{}, {}", row.len(), row.iter().join(" "));
                }
                crates
            },
        );

        // Get top of each column
        crates
            .iter()
            .map(|col| col.last().unwrap())
            .collect::<String>()
    } {
        res
    } else {
        unreachable!()
    }
}

pub fn part2(input: &str) -> String {
    if let Some(res) = try {
        let (crates, moves) = parse(input)?;

        let crates = moves.fold(
            crates,
            |mut crates, (num, from, to): (usize, usize, usize)| {
                // Move `num` crates from  crates[from] to crates[to], one by one
                let (cfrom, cto) = from_to(from, to, &mut crates);
                // Get last `num` elements of `cfrom`, reverse in place, and append to `cto`
                cto.extend(cfrom.drain(cfrom.len() - num..));
                // Print `crates`
                for row in &crates {
                    println!("{}, {}", row.len(), row.iter().join(" "));
                }
                crates
            },
        );

        // Get top of each column
        crates
            .iter()
            .map(|col| col.last().unwrap())
            .collect::<String>()
    } {
        res
    } else {
        unreachable!()
    }
}

fn from_to(from: usize, to: usize, crates: &mut [Vec<char>]) -> (&mut Vec<char>, &mut Vec<char>) {
    if from < to {
        let (a, b) = crates.split_at_mut(to);
        (&mut a[from], &mut b[0])
    } else {
        let (a, b) = crates.split_at_mut(from);
        (&mut b[0], &mut a[to])
    }
}

#[test]
fn test() {
    let input = read_input("input5.txt").unwrap();
    let input1 = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(&input), "BSDMQFLSP");
    assert_eq!(part2(&input), "PGSQBFLDP");
}
