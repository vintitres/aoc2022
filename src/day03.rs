use itertools::Itertools;
use std::collections::BTreeSet;

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

fn score(item: char) -> u32 {
    match item {
        i if i.is_ascii_lowercase() => (i as u32) - ('a' as u32) + 1,
        i if i.is_ascii_uppercase() => (i as u32) - ('A' as u32) + 27,
        _ => unimplemented!(),
    }
}

fn all3(group: impl Iterator<Item = String>) -> char {
    let (e1, e2, e3) = group
        .map(|e| BTreeSet::from_iter(e.chars()))
        .collect_tuple()
        .unwrap();
    let inter: BTreeSet<_> = e1.intersection(&e2).cloned().collect();
    *inter.intersection(&e3).next().unwrap()
}

pub fn a(input: impl Iterator<Item = String>) -> u32 {
    input.map(bothsides).map(score).sum()
}

pub fn b(input: impl Iterator<Item = String>) -> u32 {
    input.chunks(3).into_iter().map(all3).map(score).sum()
}
