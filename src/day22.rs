use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    eprintln!("{:?}", input);
    let input = input.lines().collect_vec();
    eprintln!("{:?}", input);
    input.len()
    /* 
    let (map, moves) = input.split_at(input.len() - 2);
    let moves = moves[0];
    moves.len()
    */
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day22.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
