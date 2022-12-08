use crate::utils::*;

pub fn part1(input: &str) -> usize {
    find_packet::<4>(input)
}

fn find_packet<const N: usize>(input: &str) -> usize {
    let mut input = input.as_bytes().array_windows::<N>();

    // TODO: I don't like mutability
    // STARTING HERE
    let mut count = 0;
    let mut skip = 0;
    while let Some(w) = input.nth(skip) {
        println!("{}", std::str::from_utf8(w).unwrap());
        if let Some(sk) = w
            .iter()
            .enumerate()
            .position(|(i, a)| w[i + 1..].iter().any(|b| a == b))
        {
            println!("{skip}");
            count += sk + 1;
            skip = sk;
        } else {
            break;
        }
    }

    count + N
}
fn find_packet_simple<const N: usize>(input: &str) -> usize {
    input
        .as_bytes()
        .array_windows::<N>()
        .position(|window| {
            // Return true if window has no repeated characters
            window
                .iter()
                .enumerate()
                .all(|(i, a)| window[i + 1..].iter().all(|b| a != b))
        })
        .unwrap()
        + N
}

pub fn part2(input: &str) -> usize {
    find_packet::<14>(input)
}

#[test]
fn test() {
    let input = read_input("input6.txt").unwrap();
    assert_eq!(
        part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
        7,
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    );
    assert_eq!(
        part1("bvwbjplbgvbhsrlpgdmjqwftvncz"),
        5,
        "bvwbjplbgvbhsrlpgdmjqwftvncz"
    );
    assert_eq!(
        part1("nppdvjthqldpwncqszvftbrmjlhg"),
        6,
        "nppdvjthqldpwncqszvftbrmjlhg"
    );
    assert_eq!(
        part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
        10,
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
    );
    assert_eq!(
        part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        11,
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
    );
    assert_eq!(part1(&input), 1723);
    assert_eq!(part2(&input), 3708);
}
