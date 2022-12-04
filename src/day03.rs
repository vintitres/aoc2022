use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::BTreeSet;

fn bothsides(line: &String) -> &u8 {
    let len = line.len();
    line.as_bytes()
        .chunks(len / 2)
        .into_iter()
        .map(BTreeSet::from_iter)
        .reduce(|inter, rucksack| intersect(inter, rucksack))
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn score(item: &u8) -> i32 {
    (match item {
        i if i.is_ascii_lowercase() => i - ('a' as u8) + 1,
        i if i.is_ascii_uppercase() => i - ('A' as u8) + 27,
        _ => unimplemented!(),
    }) as i32
}

fn intersect<'a>(s1: BTreeSet<&'a u8>, s2: BTreeSet<&u8>) -> BTreeSet<&'a u8> {
    s1.into_iter()
        .scan(0, |_, s1e| {
            if s2.contains(s1e) {
                Some(Some(s1e))
            } else {
                Some(None)
            }
        })
        .filter_map(|e| e)
        .collect()
}

fn all3<'a>(group: impl Iterator<Item = &'a String>) -> &'a u8 {
    group
        .map(|e| BTreeSet::from_iter(e.as_bytes()))
        .reduce(|inter, rucksack| intersect(inter, rucksack))
        .unwrap()
        .into_iter()
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
