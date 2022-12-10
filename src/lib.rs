#[cfg(feature = "day01")]
pub mod day01;
#[cfg(feature = "day02")]
pub mod day02;
#[cfg(feature = "day03")]
pub mod day03;
#[cfg(feature = "day04")]
pub mod day04;
#[cfg(feature = "day05")]
pub mod day05;
#[cfg(feature = "day06")]
pub mod day06;
#[cfg(feature = "day07")]
pub mod day07;
#[cfg(feature = "day08")]
pub mod day08;
#[cfg(feature = "day09")]
pub mod day09;
#[cfg(feature = "day10")]
pub mod day10;
#[cfg(feature = "day11")]
pub mod day11;
#[cfg(feature = "day12")]
pub mod day12;
#[cfg(feature = "day13")]
pub mod day13;
#[cfg(feature = "day14")]
pub mod day14;
#[cfg(feature = "day15")]
pub mod day15;
#[cfg(feature = "day16")]
pub mod day16;

#[cfg(feature = "my_aoc_runner")]
mod my_aoc_runner;
#[cfg(feature = "my_aoc_runner")]
use aoc_runner_derive::aoc_lib;
#[cfg(feature = "my_aoc_runner")]
aoc_lib! { year = 2022 }
