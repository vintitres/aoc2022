struct Elem {
    val: i32,
    moved: bool,
}

impl Elem {
    fn read(sval: &str) -> Elem {
        Elem {
            val: sval.parse().unwrap(),
            moved: false,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut l = LinkedList::from_iter(input.split(' ').map(Elem::read));
    let ll = l.len();
    for _ in 0..ll {
        // TODO move first not moved
    }
    vec![1000, 2000, 3000, 4000].iter().map(|i| l.nth(1000 % ll).unwrap()).product()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day20.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 15120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1234);
    }
}
