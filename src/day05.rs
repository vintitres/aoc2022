use itertools::Itertools;

const MAX_STACK: usize = 8;
const STACK_COUNT: usize = 9;
// const MAX_STACK: usize = 3;
// const STACK_COUNT: usize = 3;
fn read(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut lines = input.lines();
    let mut stacks = Vec::new();
    for _ in 0..STACK_COUNT {
        stacks.push(Vec::new());
    }
    for line in lines.by_ref().take(MAX_STACK) {
        let len = line.len();
        for i in 0..STACK_COUNT {
            if 4 * i + 1 < len {
                let c = line.chars().nth(4 * i + 1).unwrap();
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }
    }
    for mut stack in &mut stacks {
        stack.reverse()
    }
    let moves = lines.skip(2).map(|l| {
        l.split(' ').map(|w| w.parse()).flatten().collect_tuple().unwrap()
    }).collect();
    (stacks, moves)
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    for (count, from, to) in moves {
        let to = to -1;
        let from = from - 1;
        let new_len = stacks[from].len() - count;
        let moved = stacks[from][new_len..].iter().rev().cloned().collect_vec();
        stacks.get_mut(to).unwrap().extend(moved);
        stacks[from].resize(new_len, '!');
        println!("{:?}", stacks);
    }
    // format!("{:?}", stacks)
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn part2(input: &str) -> usize {
    // read(input).count()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day5.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), "");
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(input()), 865);
    // }
}
