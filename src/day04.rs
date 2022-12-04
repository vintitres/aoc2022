use aoc_runner_derive::{aoc, aoc_generator};
use text_io::scan;

#[aoc_generator(day4)]
pub fn g(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .lines()
        .map(|l| {
            let b1: i32;
            let b2: i32;
            let e1: i32;
            let e2: i32;
            scan!(l.bytes() => "{}-{},{}-{}",b1,e1,b2,e2);
            ((b1, e1), (b2, e2))
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(ranges: &[((i32, i32), (i32, i32))]) -> usize {
    ranges
        .iter()
        .filter(|((b1, e1), (b2, e2))| (b1 <= b2 && e1 >= e2) || (b1 >= b2 && e1 <= e2))
        .count()
}

#[aoc(day4, part2)]
pub fn part2(ranges: &[((i32, i32), (i32, i32))]) -> usize {
    ranges
        .iter()
        .filter(|((b1, e1), (b2, e2))| b1 <= e2 && b2 <= e1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_04a() {
        assert_eq!(part1(&g(include_str!("../input/2022/day4.txt"))), 466);
    }

    #[test]
    fn test_04b() {
        assert_eq!(part2(&g(include_str!("../input/2022/day4.txt"))), 865);
    }
}
