use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Num(u32),
}
impl Packet {
    fn read(chars: &str) -> Self {
        Self::_read(&mut chars[1..].chars())
    }
    fn _read(chars: &mut std::str::Chars) -> Self {
        let mut contents = vec![];
        let mut num = None;
        loop {
            match chars.next().unwrap() {
                '[' => contents.push(Packet::_read(chars)),
                ']' => {
                    if let Some(num) = num {
                        contents.push(Packet::Num(num));
                    };
                    return Packet::List(contents);
                }
                ',' => {
                    if let Some(num) = num {
                        contents.push(Packet::Num(num));
                    };
                    num = None;
                }
                ' ' => {}
                c if c.is_ascii_digit() => {
                    num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap())
                }
                _ => unimplemented!(),
            };
        }
    }
    fn expand(&self) -> Self {
        match self {
            Packet::Num(num) => Packet::List(vec![Packet::Num(*num)]),
            _ => unimplemented!(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l
                .iter()
                .zip_longest(r.iter())
                .map(|p| match p {
                    itertools::EitherOrBoth::Both(l, r) => l.cmp(r),
                    itertools::EitherOrBoth::Left(_) => Ordering::Greater,
                    itertools::EitherOrBoth::Right(_) => Ordering::Less,
                })
                .find(|c| *c != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            (l @ Packet::List(_), r @ Packet::Num(_)) => l.cmp(&r.expand()),
            (l @ Packet::Num(_), r @ Packet::List(_)) => l.expand().cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.take(2)
                .map(Packet::read)
                .collect_tuple::<(Packet, Packet)>()
                .unwrap()
        })
        .map(|(l, r)| l.cmp(&r))
        .enumerate()
        .filter(|(_, o)| *o != std::cmp::Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let p2 = Packet::read("[[2]]");
    let p6 = Packet::read("[[6]]");
    let pp2 = Packet::read("[[2]]");
    let pp6 = Packet::read("[[6]]");

    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::read)
        .chain(std::iter::once(p2))
        .chain(std::iter::once(p6))
        .sorted()
        .enumerate()
        .filter(|(_, p)| *p == pp2 || *p == pp6)
        .map(|(i, _)| i + 1)
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day13.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 6478);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 21922);
    }
}
