use std::collections::{BTreeSet, VecDeque};

fn doit(input: &str, cnt: usize) -> usize {
    let mut last = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        last.push_back(c);
        if i >= cnt {
            last.pop_front().unwrap();
        }
        if BTreeSet::from_iter(last.iter()).len() == cnt {
            return i + 1;
        }
    }
    unimplemented!();
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
