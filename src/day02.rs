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
        if self == op {
            3
        } else if self.counter() == *op {
            0
        } else {
            6
        }
    }
}

fn read(input: Box<dyn Iterator<Item = String>>) -> Box<dyn Iterator<Item = (char, char)>> {
    Box::new(input.map(|l| {
        let mut g = l.chars();
        let op = g.next();
        g.next();
        let st = g.next();
        (op.unwrap(), st.unwrap())
    }))
}

fn gamea((op, st): (char, char)) -> (Play, Play) {
    (Play::new(op), Play::new(st))
}

fn gameb((op, st): (char, char)) -> (Play, Play) {
    let op = Play::new(op);
    let me = match st {
        'X' => op.counter().counter(),
        'Y' => op,
        'Z' => op.counter(),
        _ => unimplemented!(),
    };
    (op, me)
}

fn score((op, me): (Play, Play)) -> i32 {
    me.points() + me.scorevs(&op)
}

pub fn a(input: Box<dyn Iterator<Item = String>>) -> i32 {
    read(input).map(gamea).map(score).sum()
}

pub fn b(input: Box<dyn Iterator<Item = String>>) -> i32 {
    read(input).map(gameb).map(score).sum()
}
