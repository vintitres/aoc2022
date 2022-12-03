use itertools::Itertools;
use std::collections::BTreeSet;
use aoc_runner_derive::{aoc_generator,aoc};

fn bothsides(line: String) -> char {
    let len = line.len();
    let (l, r) = line
        .chars()
        .chunks(len / 2)
        .into_iter()
        .map(BTreeSet::from_iter)
        .collect_tuple()
        .unwrap();
    *l.intersection(&r).next().unwrap()
}

fn score(item: char) -> i32 {
    (match item as u8 {
        i if i.is_ascii_lowercase() => i - ('a' as u8) + 1,
        i if i.is_ascii_uppercase() => i - ('A' as u8) + 27,
        _ => unimplemented!(),
    }) as i32
}

fn all3(group: impl Iterator<Item = String>) -> char {
    let (e1, e2, e3) = group
        .map(|e| BTreeSet::from_iter(e.chars()))
        .collect_tuple()
        .unwrap();
    *e1.intersection(&e2).cloned().collect::<BTreeSet<_>>().intersection(&e3).next().unwrap()
}

pub fn a(input_lines: impl Iterator<Item = String>) -> i32 {
    input_lines.map(bothsides).map(score).sum()
}

pub fn b(input_lines: impl Iterator<Item = String>) -> i32 {
    input_lines.chunks(3).into_iter().map(all3).map(score).sum()
}

#[aoc_generator(day3)]
pub fn g(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day3, part1)]
pub fn part1(input_lines: &[String]) -> i32 {
    a(input_lines.iter().map(String::from))
}

#[aoc(day3, part2)]
pub fn part2(input_lines: &[String]) -> i32 {
    b(input_lines.iter().map(String::from))
}
