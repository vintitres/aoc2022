use aoc_runner_derive::aoc;
use itertools::Itertools;
use text_io::scan;

struct Range {
    begin: i32,
    end: i32,
}

impl Range {
    fn from_str(s: &str) -> Range {
        let begin: i32;
        let end: i32;
        scan!(s.bytes() => "{}-{}",begin,end);
        Range { begin, end }
    }

    fn contains(&self, other: &Self) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }

    fn touches(&self, other: &Self) -> bool {
        self.begin <= other.end && other.begin <= self.end
    }
}

fn read(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input
        .lines()
        .map(|l| l.split(",").map(Range::from_str).collect_tuple().unwrap())
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    read(input)
        .filter(|(l, r)| l.contains(r) || r.contains(l))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    read(input).filter(|(l, r)| l.touches(r)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day4.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 466);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 865);
    }
}
