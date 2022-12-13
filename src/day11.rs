use std::collections::VecDeque;

use itertools::Itertools;

struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    testdiv: usize,
    iftrue: usize,
    iffalse: usize,
    inspections: usize,
}

impl Monkey {
    fn inspect(&mut self, worrydiv: usize, worrymod: usize) -> Vec<(usize, usize)> {
        let mut throws = Vec::new();
        loop {
            let item = match self.items.pop_front() {
                None => break,
                Some(item) => item,
            };
            self.inspections += 1;
            let item = (self.op.calc(item) / worrydiv) % worrymod;
            let to_monkey = if item % self.testdiv == 0 {
                self.iftrue
            } else {
                self.iffalse
            };
            throws.push((to_monkey, item));
        }
        throws
    }
}

enum NumOp {
    Add,
    Mul,
}

enum Val {
    Old,
    Num(usize),
}

struct Op {
    op: NumOp,
    val: Val,
}

impl Op {
    fn read(input: &str) -> Self {
        let (_l, op, val) = input.split(' ').collect_tuple().unwrap();
        let op = match op {
            "+" => NumOp::Add,
            "*" => NumOp::Mul,
            _ => unimplemented!(),
        };
        let val = match val {
            "old" => Val::Old,
            val => Val::Num(val.parse().unwrap()),
        };
        Op { op, val }
    }
    fn calc(&self, x: usize) -> usize {
        let val = match self.val {
            Val::Num(val) => val,
            Val::Old => x,
        };
        match self.op {
            NumOp::Add => x + val,
            NumOp::Mul => x * val,
        }
    }
}

fn read_monkey<'a>(lines: impl Iterator<Item = &'a str>) -> Monkey {
    let (_, items, op, test, iftrue, iffalse) = lines.take(6).collect_tuple().unwrap();
    let items = items["  Starting items: ".len()..]
        .split(", ")
        .map(|i| i.parse::<usize>().unwrap());
    let op = Op::read(&op["  Operation: new = ".len()..]);
    let testdiv = test["  Test: divisible by ".len()..]
        .parse::<usize>()
        .unwrap();
    let iftrue = iftrue["    If true: throw to monkey ".len()..]
        .parse::<usize>()
        .unwrap();
    let iffalse = iffalse["    If false: throw to monkey ".len()..]
        .parse::<usize>()
        .unwrap();
    Monkey {
        items: VecDeque::from_iter(items),
        op,
        testdiv,
        iftrue,
        iffalse,
        inspections: 0,
    }
}

fn simulate(input: &str, rounds: usize, worrydiv: usize) -> usize {
    let mut monkeys = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(read_monkey)
        .collect_vec();
    let worrymod = monkeys.iter().map(|m| m.testdiv).fold(1, |a, b| a * b);
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let throws = monkeys.get_mut(i).unwrap().inspect(worrydiv, worrymod);
            for (tomonkey, item) in throws {
                monkeys[tomonkey].items.push_back(item);
            }
        }
    }
    let (m1, m2) = monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();
    m1 * m2
}

pub fn part1(input: &str) -> usize {
    simulate(input, 20, 3)
}

pub fn part2(input: &str) -> usize {
    simulate(input, 10000, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day11.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 117624);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 16792940265);
    }
}
