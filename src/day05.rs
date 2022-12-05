use itertools::Itertools;

type Stack = Vec<char>;
type Move = (usize, usize, usize);

fn read(input: &str) -> (Vec<Stack>, Vec<Move>) {
    const MAX_STACK: usize = 8;
    const STACK_COUNT: usize = 9;
    let mut lines = input.lines();
    let mut stacks = vec![Vec::new(); STACK_COUNT];
    for line in lines.by_ref().take(MAX_STACK) {
        for (i, stack) in stacks.iter_mut().enumerate() {
            if let c = line.chars().nth(4 * i + 1).unwrap() {
                if c != ' ' {
                    stack.push(c);
                }
            }
        }
    }
    stacks.iter_mut().for_each(|s| s.reverse());
    let moves = lines
        .skip(2)
        .map(|l| {
            l.split(' ')
                .flat_map(|w| w.parse())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    (stacks, moves)
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    moves.iter().for_each(|(count, from, to)| {
        let to = to - 1;
        let from = from - 1;
        let new_len = stacks[from].len() - count;
        let moved = stacks[from][new_len..].iter().rev().cloned().collect_vec();
        stacks.get_mut(to).unwrap().extend(moved);
        stacks[from].resize(new_len, '!');
        println!("{:?}", stacks);
    });
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    moves.iter().for_each(|(count, from, to)| {
        let to = to - 1;
        let from = from - 1;
        let new_len = stacks[from].len() - count;
        let moved = stacks[from][new_len..].iter().cloned().collect_vec();
        stacks.get_mut(to).unwrap().extend(moved);
        stacks[from].resize(new_len, '!');
        println!("{:?}", stacks);
    });
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day5.txt")
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
