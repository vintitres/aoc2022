pub fn part1(input: &str) -> usize {
    input.len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day23.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
