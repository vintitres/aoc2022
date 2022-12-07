use std::{collections::BTreeMap, iter};

fn doit(input: &str, prelen: usize) -> usize {
    const PLACEHOLDER: char = '!';
    let mut lastcnt = BTreeMap::new();
    for (i, (b, e)) in iter::repeat(PLACEHOLDER)
        .take(prelen)
        .chain(input.chars())
        .zip(input.chars())
        .enumerate()
    {
        lastcnt.entry(e).and_modify(|cnt| *cnt += 1).or_insert(1);
        if b != PLACEHOLDER && *lastcnt.entry(b).and_modify(|cnt| *cnt -= 1).or_default() == 0 {
            lastcnt.remove(&b);
        }
        if lastcnt.len() == prelen {
            return i + 1;
        }
    }
    panic!();
}

pub fn part1(input: &str) -> usize {
    doit(input, 4)
}

pub fn part2(input: &str) -> usize {
    doit(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day6.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1816);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 2625);
    }
}
