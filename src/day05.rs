use itertools::Itertools;

type Stack = Vec<char>;
struct Stacks {
    stacks: Vec<Stack>,
}
type Move = (usize, usize, usize);

fn read(input: &str) -> (Stacks, Vec<Move>) {
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
        .collect();
    (Stacks { stacks }, moves)
}

impl Stacks {
    pub fn domove(&mut self, (count, from, to): &Move) {
        // let mut from = self[from - 1];
        // let mut to = self[to - 1];
        let to = to - 1;
        let from = from - 1;
        let new_len = self.stacks[from].len() - count;
        let moved = self.stacks[from][new_len..]
            .iter()
            .rev()
            .cloned()
            .collect_vec();
        self.stacks.get_mut(to).unwrap().extend(moved);
        self.stacks[from].resize(new_len, '!');
    }
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    moves.iter().for_each(|m| {
        stacks.domove(m);
    });
    stacks
        .stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = read(input);
    moves.iter().for_each(|(count, from, to)| {
        let to = to - 1;
        let from = from - 1;
        let new_len = stacks.stacks[from].len() - count;
        let moved = stacks.stacks[from][new_len..].iter().cloned().collect_vec();
        stacks.stacks.get_mut(to).unwrap().extend(moved);
        stacks.stacks[from].resize(new_len, '!');
    });
    stacks
        .stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
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
