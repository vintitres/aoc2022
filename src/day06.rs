fn read(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &str) -> String {
    let data = read(input);
    String::from(*data.first().unwrap())
}

pub fn part2(input: &str) -> String {
    let data = read(input);
    String::from(*data.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day6.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), "MQSHJMWNH");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), "LLWJRBHVZ");
    }
}
