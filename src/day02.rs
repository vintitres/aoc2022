use text_io::read;

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

fn read(line: String) -> (char, char) {
    let mut line = line.bytes();
    (read!("{}", line), read!("{}", line))
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

pub fn a(input: impl Iterator<Item = String>) -> i32 {
    input.map(read).map(gamea).map(score).sum()
}

pub fn b(input: impl Iterator<Item = String>) -> i32 {
    input.map(read).map(gameb).map(score).sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    fn input(t: &str) -> Box<dyn Iterator<Item = String>> {
        Box::new(
            io::BufReader::new(File::open(format!("input/{}", t)).unwrap())
                .lines()
                .map(|l| l.unwrap()),
        )
    }

    #[test]
    fn test_02a() {
        assert_eq!(a(input("02")), 14827);
    }

    #[test]
    fn test_02b() {
        assert_eq!(b(input("02")), 13889);
    }
}
