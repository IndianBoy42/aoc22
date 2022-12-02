#![feature(bench_black_box)]
use std::hint::black_box;

use aoc22::utils::read_input;
use aoc22::*;

macro_rules! run_day {
    ($n:ident, $i:tt) => {{
        let input = include_str!(concat!("../", "input", $i, ".txt"));
        $n::part2(black_box(input));
        $n::part2(black_box(input));
        input
    }};
}

macro_rules! run_all_days {
    ($n:ident) => { {
        $n::part1(black_box(&$n));
        $n::part1(black_box(&$n));
    } };
    ($n:ident, $($ns:ident),+) => { {
        run_all_days!($n);
        run_all_days!($($ns),+);
    } };
}

// fn main() {
//     let day1 = run_day!(day1, 1);
//     let day2 = run_day!(day2, 2);
//     let day3 = run_day!(day3, 3);
//     let day4 = run_day!(day4, 4);
//     let day5 = run_day!(day5, 5);
//     let day6 = run_day!(day6, 6);
//     let day7 = run_day!(day7, 7);
//     let day8 = run_day!(day8, 8);
//     let day9 = run_day!(day9, 9);
//     let day10 = run_day!(day10, 10);
//     let day11 = run_day!(day11, 11);
//     let day12 = run_day!(day12, 12);
//     let day13 = run_day!(day13, 13);
//     let day14 = run_day!(day14, 14);
//
//     run_all_days!(
//         day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14
//     )
// }
fn main() {
    let input = read_input("input1.txt").unwrap();
    assert_eq!(day1::part1(&input), 0);
    assert_eq!(day1::part2(&input), 0);
}
