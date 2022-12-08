use crate::utils::*;

// A,X - Rock
// B,Y - Paper
// C,Z - Scissors

fn winner(me: char) -> char {
    match me {
        'A' | 'X' => 'Y',
        'B' | 'Y' => 'Z',
        'C' | 'Z' => 'X',
        _ => panic!("Invalid input"),
    }
}
fn loser(me: char) -> char {
    winner(winner(me))
}
fn score(me: char) -> u32 {
    match me {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!("Invalid input"),
    }
}

pub fn part1(input: &str) -> u32 {
    fn rps(opp: char, me: char) -> u32 {
        score(me)
            + match (opp, me) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => panic!("Invalid input"),
            }
    }
    input
        .lines()
        .map(|line| line.chars().next_tuple().unwrap())
        .map(|(opp, _, me)| rps(opp, me))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    // X means you need to lose
    // Y means you need to draw
    // Z means you need to win
    fn rps(opp: char, res: char) -> u32 {
        match res {
            'X' => score(loser(opp)),
            'Y' => 3 + score(opp),
            'Z' => 6 + score(winner(opp)),
            _ => panic!("Invalid input {}", res),
        }
    }
    input
        .lines()
        .map(|line| line.chars().next_tuple().unwrap())
        .map(|(opp, _, res)| rps(opp, res))
        .sum()
}

#[test]
fn test() {
    let input = read_input("input2.txt").unwrap();
    assert_eq!(part1(&input), 10624);
    assert_eq!(part2(&input), 14060);
}
