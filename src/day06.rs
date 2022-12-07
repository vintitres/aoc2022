use std::collections::{BTreeMap, VecDeque};

fn doit(input: &str, prelen: usize) -> usize {
    let mut lastcnt = BTreeMap::from_iter(input.chars().take(prelen).map(|c| (c,1)));
    for (b, e) in input.chars().zip(input.chars().skip(prelen)) {
        lastcnt.entry(e).and_modify(|cnt| *cnt += 1).or_insert(1);
        if i >= prelen {
            if *lastcnt.entry(b).and_modify(|cnt| *cnt -= 1).or_default() == 0 {
                lastcnt.remove(&b);
            }
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
