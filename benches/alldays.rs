#![feature(concat_idents)]
use aoc21::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_day {
    ($c:ident, $n:ident, $i:tt) => {{
        let input = include_str!(concat!("../", "input", $i, ".txt"));
        $c.bench_function(concat!("day", $i, "p1"), |b| {
            b.iter(|| $n::part1(black_box(input)))
        });
        $c.bench_function(concat!("day", $i, "p2"), |b| {
            b.iter(|| $n::part2(black_box(input)))
        });
        $c.bench_function(concat!("day", $i), |b| {
            b.iter(|| {
                $n::part2(black_box(input));
                $n::part2(black_box(input))
            })
        });
        input
    }};
}

macro_rules! bench_all_days {
    ($n:ident) => { {
        $n::part1(black_box(&$n));
        $n::part1(black_box(&$n));
    } };
    ($n:ident, $($ns:ident),+) => { {
        bench_all_days!($n);
        bench_all_days!($($ns),+);
    } };
}

pub fn criterion_benchmark(crit: &mut Criterion) {
    let mut c = crit.benchmark_group("Main Benchmarks");
    c.sample_size(100);

    let day1 = bench_day!(c, day1, 1);
    let day2 = bench_day!(c, day2, 2);
    let day3 = bench_day!(c, day3, 3);
    let day4 = bench_day!(c, day4, 4);
    let day5 = bench_day!(c, day5, 5);
    let day6 = bench_day!(c, day6, 6);
    let day7 = bench_day!(c, day7, 7);
    let day8 = bench_day!(c, day8, 8);
    let day9 = bench_day!(c, day9, 9);
    let day10 = bench_day!(c, day10, 10);
    let day11 = bench_day!(c, day11, 11);
    let day12 = bench_day!(c, day12, 12);
    let day13 = bench_day!(c, day13, 13);
    let day14 = bench_day!(c, day14, 14);
    let day14 = bench_day!(c, day14, 14);
    let day15 = bench_day!(c, day15, 15);
    let day16 = bench_day!(c, day16, 16);
    c.bench_function("alldays", |b| {
        b.iter(|| {
            bench_all_days!(
                day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
                day14, day15, day16
            )
        })
    });
}

macro_rules! iai_day {
    ($b:ident, $n:ident, $i:tt) => {
        fn $b() -> usize {
            let input = include_str!(concat!("../", "input", $i, ".txt"));
            $n::part1(black_box(input));
            $n::part2(black_box(input))
        }
    };
}

// iai_day!(bench_day1, day1, 1);
// iai_day!(bench_day2, day2, 2);
// iai_day!(bench_day3, day3, 3);
// iai_day!(bench_day4, day4, 4);
// iai_day!(bench_day5, day5, 5);
// iai::main!(bench_day1, bench_day2, bench_day3, bench_day4, bench_day5);

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
