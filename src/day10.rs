use crate::utils::*;

const CHECK: [usize; 6] = [20, 60, 100, 140, 180, 220];
pub fn part1(input: &str) -> isize {
    run(input)
        // .inspect(|(i, x)| println!("@{i} = {x}"))
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(i, x)| i as isize * x)
        .sum()
}

fn run(input: &str) -> impl Iterator<Item = (usize, isize)> + '_ {
    // instructions:
    // noop - do nothing - takes 1 cycle
    // addx {integer} - add x to the register - takes 2 cycles
    input
        .lines()
        .scan((0, 1), |(cyccnt, reg), instr| {
            // dbg!(instr);
            Some(if "noop" == instr {
                *cyccnt += 1;
                Some((*cyccnt, *reg)).into_iter().chain(None)
            } else {
                let n = instr[5..].parse::<isize>().unwrap();
                *cyccnt += 2;
                let first = Some((*cyccnt - 1, *reg));
                let second = Some((*cyccnt, *reg));
                *reg += n;
                first.into_iter().chain(second)
            })
        })
        .flatten()
}

const WIDTH: usize = 40;
pub fn part2(input: &str) -> String {
    let seq = run(input);
    let img = run(input)
        .map(|(i, x)| {
            println!("{i}->{x}");
            if (((i - 1) % WIDTH) as isize - x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect_vec();
    let img = img
        .chunks(WIDTH)
        .map(|row| row.iter().copied().join(""))
        .join("\n");

    println!("{img}");

    img
}

fn find_letters(input: &str) -> String {
    // Every character is five pixels wide
    const CHAR_WIDTH: usize = 4;
    fn get_letter(input: &str, i: usize) -> String {
        input
            .as_bytes()
            .chunks(WIDTH + 1)
            .map(|row| &row[i * (CHAR_WIDTH + 1)..][..CHAR_WIDTH])
            .map(|row| std::str::from_utf8(row).unwrap())
            .join("\n")
    }

    (0..8)
        .map(|i| dbg!(get_letter(input, i)))
        .map(|ch| {
            CHARS
                .iter()
                .find_map(|(pat, out)| (pat == &ch).as_some(out))
                .unwrap()
        })
        .join("")
}
#[test]
fn test() {
    let input = read_input("input10.txt").unwrap();
    assert_eq!(part1(&input), 16480);
    // let input = read_input("test.txt").unwrap();
    let expected = "###..#....####.####.#..#.#....###..###..
#..#.#....#....#....#..#.#....#..#.#..#.
#..#.#....###..###..#..#.#....#..#.###..
###..#....#....#....#..#.#....###..#..#.
#....#....#....#....#..#.#....#....#..#.
#....####.####.#.....##..####.#....###.."; // PLEFULPB
    assert_eq!(part2(&input), expected);
    assert_eq!(find_letters(&part2(&input)), "PLEFULPB");
}

const CHAR_P: &str = "###.
#..#
#..#
###.
#...
#...";
const CHAR_F: &str = "####
#...
###.
#...
#...
#...";
const CHAR_L: &str = "#...
#...
#...
#...
#...
####";
const CHAR_E: &str = "####
#...
###.
#...
#...
####";
const CHAR_U: &str = "#..#
#..#
#..#
#..#
#..#
.##.";
const CHAR_B: &str = "###.
#..#
###.
#..#
#..#
###.";
const CHAR_I: &str = "####.
..#..
..#..
..#..
..#..
####.";
const CHAR_N: &str = "#..#.
##.#.
#.##.
#..#.
#..#.
#..#.";
const CHAR_S: &str = "####
#....
####
....#
....#
####";
const CHAR_A: &str = "####.
#..#.
#..#.
####.
#..#.
#..#.";
const CHAR_R: &str = "###.
#..#
###.
#.#.
#..#
#..#.";
const CHAR_H: &str = "#..#
#..#
####
#..#
#..#
#..#.";
const CHAR_C: &str = "####
#....
#....
#....
#....
####";
const CHAR_T: &str = "####
..#..
..#..
..#..
..#..
..#..";
const CHAR_M: &str = "#...#
##.##
#.#.#
#...#
#...#
#...#";
const CHAR_D: &str = "###.
#..#
#..#
#..#
#..#
###.";
const CHAR_G: &str = "####
#....
#....
#..##
#..#
####";
const CHAR_O: &str = "####
#..#
#..#
#..#
#..#
####";
const CHAR_K: &str = "#..#
#.#.
##..
#.#.
#..#
#..#.";
const CHAR_V: &str = "#...#
#...#
#...#
.#.#.
..#..
..#..";
const CHAR_J: &str = "....#
....#
....#
....#
#...#
####.";
const CHAR_Z: &str = "####
....#
..#..
.#...
#....
####";
const CHAR_W: &str = "#...#
#...#
#.#.#
##.##
#...#
#...#";
const CHAR_Y: &str = "#...#
#...#
.#.#.
..#..
..#..
..#..";
const CHAR_X: &str = "#...#
.#.#.
..#..
.#.#.
#...#
#...#";
const CHAR_Q: &str = "####
#..#
#..#
#..#
#..#
####";
const CHAR_0: &str = "####.
#..#.
#..#.
#..#.
#..#.
####.";
const CHAR_1: &str = "..#..
.##..
..#..
..#..
..#..
####.";
const CHAR_2: &str = "####.
....#
####.
#....
#....
####.";
const CHARS: &[(&str, &str)] = &[
    (CHAR_P, "P"),
    (CHAR_F, "F"),
    (CHAR_L, "L"),
    (CHAR_E, "E"),
    (CHAR_U, "U"),
    (CHAR_B, "B"),
    (CHAR_I, "I"),
    (CHAR_N, "N"),
    (CHAR_S, "S"),
    (CHAR_A, "A"),
    (CHAR_R, "R"),
    (CHAR_H, "H"),
    (CHAR_C, "C"),
    (CHAR_T, "T"),
    (CHAR_M, "M"),
    (CHAR_D, "D"),
    (CHAR_G, "G"),
    (CHAR_O, "O"),
    (CHAR_K, "K"),
    (CHAR_V, "V"),
    (CHAR_J, "J"),
    (CHAR_Z, "Z"),
    (CHAR_W, "W"),
    (CHAR_Y, "Y"),
    (CHAR_X, "X"),
    (CHAR_Q, "Q"),
    (CHAR_0, "0"),
    (CHAR_1, "1"),
    (CHAR_2, "2"),
];
