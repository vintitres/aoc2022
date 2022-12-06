// use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(PartialEq, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scisors,
}

impl Play {
    pub fn new(p: char) -> Play {
        match p {
            'A' => Play::Rock,
            'X' => Play::Rock,
            'B' => Play::Paper,
            'Y' => Play::Paper,
            'C' => Play::Scisors,
            'Z' => Play::Scisors,
            _ => unimplemented!(),
        }
    }

    fn points(&self) -> i32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scisors => 3,
        }
    }
    fn counter(&self) -> Self {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scisors,
            Play::Scisors => Play::Rock,
        }
    }

    fn scorevs(&self, op: &Self) -> i32 {
        match (*self, *op) {
            (me, op) if me == op => 3,           // draw
            (me, op) if me.counter() == op => 0, // lose
            _ => 6,                              // win
        }
    }
}

fn read(line: &str) -> (char, char) {
    line.split(' ')
        .map(|s| s.chars().next().unwrap())
        .collect_tuple()
        .unwrap()
}

fn gamea((op, st): (char, char)) -> (Play, Play) {
    (Play::new(op), Play::new(st))
}

fn gameb((op, st): (char, char)) -> (Play, Play) {
    let op = Play::new(op);
    let me = match st {
        'X' => op.counter().counter(), // lose
        'Y' => op,                     // draw
        'Z' => op.counter(),           // win
        _ => unimplemented!(),
    };
    (op, me)
}

fn score((op, me): (Play, Play)) -> i32 {
    me.points() + me.scorevs(&op)
}

// #[aoc(day2, part1)]
pub fn a(input: &str) -> i32 {
    input.lines().map(read).map(gamea).map(score).sum()
}

// #[aoc(day2, part2)]
pub fn b(input: &str) -> i32 {
    input.lines().map(read).map(gameb).map(score).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day2.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(a(input()), 14827);
    }

    #[test]
    fn test_part2() {
        assert_eq!(b(input()), 13889);
    }
}
