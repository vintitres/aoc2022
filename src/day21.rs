use std::collections::BTreeMap;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
enum Monkey {
    Num(i64),
    Calc(String, Op, String),
}

impl Monkey {
    fn val(&self, monkeys: &BTreeMap<String, Monkey>) -> i64 {
        match self {
            Self::Num(v) => *v,
            Self::Calc(l, op, r) => {
                let ll = monkeys.get(l).unwrap().val(monkeys);
                let rr = monkeys.get(r).unwrap().val(monkeys);
                match op {
                    Op::Add => ll + rr,
                    Op::Sub => ll - rr,
                    Op::Mul => ll * rr,
                    Op::Div => ll / rr,
                }
            }
        }
    }
    fn _read(shout: &str) -> Monkey {
        let mut shout = shout.split(' ');
        let l = shout.next().unwrap();
        match shout.next() {
            None => Monkey::Num(l.parse().unwrap()),
            Some(op) => {
                let r = shout.next().unwrap();
                let op = match op {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => unimplemented!(),
                };
                Monkey::Calc(l.to_string(), op, r.to_string())
            }
        }
    }
    fn read(line: &str) -> (String, Monkey) {
        let (name, shout) = line.split_once(": ").unwrap();
        (name.to_string(), Monkey::_read(shout))
    }
}

pub fn part1(input: &str) -> i64 {
    let monkeys = BTreeMap::from_iter(input.lines().map(Monkey::read));
    monkeys.get("root").unwrap().val(&monkeys)
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day21.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 158661812617812);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1234);
    }
}
