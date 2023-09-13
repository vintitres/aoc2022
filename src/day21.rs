use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Num(i64),
    Calc(String, Op, String),
    Calc2(Box<Monkey>, Op, Box<Monkey>),
    X,
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
            Self::Calc2(_, _, _) => unimplemented!(),
            Self::X => unimplemented!(),
        }
    }
    fn simpl(&self, monkeys: &BTreeMap<String, Monkey>) -> Monkey {
        match self {
            n @ Self::Num(_) => n.clone(),
            Self::Calc(l, op, r) => {
                let ll = monkeys.get(l).unwrap().simpl(monkeys);
                let rr = monkeys.get(r).unwrap().simpl(monkeys);
                match (ll, rr) {
                    (Monkey::Num(ll), Monkey::Num(rr)) => Monkey::Num(match op {
                        Op::Add => ll + rr,
                        Op::Sub => ll - rr,
                        Op::Mul => ll * rr,
                        Op::Div => ll / rr,
                    }),
                    (l, r) => Monkey::Calc2(
                        Box::new(l.simpl(monkeys)),
                        op.clone(),
                        Box::new(r.simpl(monkeys)),
                    ),
                }
            }
            c2 @ Self::Calc2(_, _, _) => c2.clone(),
            Self::X => Self::X,
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
    fn solve(&self, eq: i64) -> i64 {
        match self {
            Monkey::X => eq,
            Monkey::Calc2(l, op, r) => {
                if let Monkey::Num(ll) = l.as_ref() {
                    match op {
                        Op::Add => r.solve(eq - ll), // ll + x = eq
                        Op::Sub => r.solve(ll - eq), // ll - x = eq
                        Op::Mul => r.solve(eq / ll), // ll * x = eq
                        Op::Div => r.solve(ll / eq), // ll / x = eq  -> ll = eq * x
                    }
                } else if let Monkey::Num(rr) = r.as_ref() {
                    match op {
                        Op::Add => l.solve(eq - rr), // x + rr = eq
                        Op::Sub => l.solve(eq + rr), // x - rr = eq
                        Op::Mul => l.solve(eq / rr), // x * rr = eq
                        Op::Div => l.solve(eq * rr), // x / rr = eq
                    }
                } else {
                    unimplemented!();
                }
            }
            _ => unimplemented!(),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let monkeys = BTreeMap::from_iter(input.lines().map(Monkey::read));
    monkeys.get("root").unwrap().val(&monkeys)
}

pub fn part2(input: &str) -> i64 {
    let mut monkeys = BTreeMap::from_iter(input.lines().map(Monkey::read));
    monkeys
        .entry("humn".to_string())
        .and_modify(|m| *m = Monkey::X);
    match monkeys.get("root").unwrap() {
        Monkey::Calc(l, _, r) => {
            let r = match monkeys.get(r).unwrap().simpl(&monkeys) {
                Monkey::Num(n) => n,
                _ => unimplemented!(),
            };
            monkeys.get(l).unwrap().simpl(&monkeys).solve(r)
        }
        _ => unimplemented!(),
    }
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
        assert_eq!(part2(input()), 3352886133831);
    }
}
