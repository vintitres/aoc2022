pub fn part1(input: &str) -> u32 {
    1
}

pub fn part2(input: &str) -> u32 {
    1 + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 3);
    }
}
