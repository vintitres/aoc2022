use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Chunk;
use itertools::Itertools;
use std::collections::BTreeSet;

fn bothsides(line: &String) -> char {
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

// TODO siplier signature for the chunk
fn all3(group: Chunk<'_, std::slice::Iter<'_, String>>) -> char {
    let (e1, e2, e3) = group
        .map(|e| BTreeSet::from_iter(e.chars()))
        .collect_tuple()
        .unwrap();
    *e1.intersection(&e2)
        .cloned()
        .collect::<BTreeSet<_>>()
        .intersection(&e3)
        .next()
        .unwrap()
}

pub fn a(input_lines: impl Iterator<Item = String>) -> i32 {
    let v: Vec<String> = input_lines.collect();
    part1(&v)
}

pub fn b(input_lines: impl Iterator<Item = String>) -> i32 {
    let v: Vec<String> = input_lines.collect();
    part2(&v)
}

#[aoc_generator(day3)]
pub fn g(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day3, part1)]
pub fn part1(input_lines: &[String]) -> i32 {
    input_lines.iter().map(bothsides).map(score).sum()
}

#[aoc(day3, part2)]
pub fn part2(input_lines: &[String]) -> i32 {
    input_lines
        .iter()
        .chunks(3)
        .into_iter()
        .map(all3)
        .map(score)
        .sum()
}
