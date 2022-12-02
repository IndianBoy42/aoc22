#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::inline_always)]
#![allow(clippy::must_use_candidate)]
#![feature(try_blocks)]
#![feature(associated_type_bounds)]
// #![feature(const_generic_impls_guard)]
#![feature(iter_partition_in_place)]
#![feature(map_entry_replace)]
#![feature(specialization)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_uninit_array)]
#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(bool_to_option)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(hash_drain_filter)]
#![feature(trusted_len)]
#![feature(stmt_expr_attributes)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

pub mod grid;
pub mod searcher;
pub mod u32set;
pub mod utils;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
// pub mod day18;
pub mod day22;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
