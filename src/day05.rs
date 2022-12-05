use itertools::Itertools;

type Stack = Vec<char>;
type Move = (usize, usize, usize);

fn read(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut lines = input.lines();
    let mut stacks = Vec::new();
    for line in lines.by_ref().take_while(|l| !l.starts_with(" 1")) {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| {
                if stacks.len() < i + 1 {
                    stacks.resize(i + 1, Vec::new())
                }
                stacks[i].push(c)
            });
    }
    stacks.iter_mut().for_each(|s| s.reverse());
    let moves = lines
        .skip(1)
        .map(|l| {
            l.split(' ')
                .flat_map(|w| w.parse())
                .collect_tuple()
                .unwrap()
        })
        .map(|(count, from, to)| (count, from - 1, to - 1))
        .collect();
    (stacks, moves)
}

fn domove(stacks: &mut [Stack], (count, from, to): Move, onegrab: bool) {
    let new_len = stacks[from].len() - count;
    let moved = &stacks[from][new_len..];
    // TODO no cloned
    let moved = if onegrab {
        moved.iter().cloned().collect_vec()
    } else {
        moved.iter().rev().cloned().collect_vec()
    };
    stacks[to].extend(moved);
    stacks[from].resize(new_len, '!');
}

fn tops(stacks: &[Stack]) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    moves.iter().for_each(|m| domove(&mut stacks, *m, false));
    tops(&stacks)
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    moves.iter().for_each(|m| domove(&mut stacks, *m, true));
    tops(&stacks)
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
