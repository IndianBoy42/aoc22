use crate::utils::*;

type Snailfish = Vec<(u8, u8)>;

fn parse(input: &str) -> impl IntoIterator<Item = Snailfish> + '_' {
    input.lines().map(|line| {
        line.bytes()
            .fold((0, Snailfish::new()), |(dep, mut acc), c| match c {
                b'[' => (dep + 1, acc),
                b']' => (dep - 1, acc),
                c @ b'0'..=b'9' => {
                    acc.push((c, dep));
                    (dep, acc)
                }
                _ => 
            })
            .1
    })
}

pub fn part1(input: &str) -> usize {
    unimplemented!()
}

pub fn part2(input: &str) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input0.txt").unwrap();
    assert_eq!(part1(&input), 0);
    assert_eq!(part2(&input), 0);
}
