// use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::BTreeSet;

fn bothsides(line: &str) -> &u8 {
    let len = line.len();
    line.as_bytes()
        .chunks(len / 2)
        .into_iter()
        .map(BTreeSet::from_iter)
        .reduce(intersect)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn score(item: &u8) -> i32 {
    (match item {
        i if i.is_ascii_lowercase() => i - b'a' + 1,
        i if i.is_ascii_uppercase() => i - b'A' + 27,
        _ => unimplemented!(),
    }) as i32
}

fn intersect<'a>(s1: BTreeSet<&'a u8>, s2: BTreeSet<&u8>) -> BTreeSet<&'a u8> {
    s1.into_iter()
        .scan(0, |_, s1e| {
            Some(if s2.contains(s1e) { Some(s1e) } else { None })
        })
        .flatten()
        .collect()
}

fn all3<'a>(group: impl Iterator<Item = &'a str>) -> &'a u8 {
    group
        .map(|e| BTreeSet::from_iter(e.as_bytes()))
        .reduce(intersect)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

// #[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(bothsides).map(score).sum()
}

// #[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(all3)
        .map(score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day3.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 8394);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 2413);
    }
}
