use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Num(u32),
}
impl Packet {
    fn read(chars: &str) -> Self {
        println!("{:?}", chars);
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
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("{:?} {:?}", self, other);
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
            (l, Packet::Num(r)) => l.cmp(&Packet::List(vec![Packet::Num(*r)])),
            (Packet::Num(l), r) => Packet::List(vec![Packet::Num(*l)]).cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.eq(r),
            (Packet::List(l), Packet::List(r)) => l
                .iter()
                .zip_longest(r.iter())
                .map(|p| match p {
                    itertools::EitherOrBoth::Both(l, r) => l.eq(r),
                    itertools::EitherOrBoth::Left(_) => false,
                    itertools::EitherOrBoth::Right(_) => false,
                })
                .find(|c| !(*c))
                .unwrap_or(true),
            (l, Packet::Num(r)) => l.eq(&Packet::List(vec![Packet::Num(*r)])),
            (Packet::Num(l), r) => Packet::List(vec![Packet::Num(*l)]).eq(r),
        }
    }
}

impl Eq for Packet {}

pub fn read(_input: &str) -> usize {
    1
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        // .skip(2).take(1)
        .map(|c| {
            let (l, r) = c
                .take(2)
                .map(Packet::read)
                .collect_tuple::<(Packet, Packet)>()
                .unwrap();
            l.cmp(&r)
        })
        .enumerate()
        .inspect(|v| println!("{:?}", v))
        .filter(|(_, o)| *o != std::cmp::Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    read(input)
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
        assert_eq!(part2(input()), 1);
    }
}
