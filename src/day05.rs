fn read(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines()
}

pub fn part1(input: &str) -> usize {
    read(input).count()
}

pub fn part2(input: &str) -> usize {
    read(input).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day5.txt")
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
