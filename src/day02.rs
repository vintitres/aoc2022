pub fn a(input: Box<dyn Iterator<Item = String>>) -> i32 {
    input
        .map(|l| {
            let mut g = l.chars();
            let op = g.next();
            let op = op.unwrap();
            g.next();
            let me = g.next();
            let me = me.unwrap();
            match me {
                'X' => {
                    1 + match op {
                        'A' => 3,
                        'B' => 0,
                        'C' => 6,
                        c => panic!("{}", c),
                    }
                }
                'Y' => {
                    2 + match op {
                        'A' => 6,
                        'B' => 3,
                        'C' => 0,
                        c => panic!("{}", c),
                    }
                }
                'Z' => {
                    3 + match op {
                        'A' => 0,
                        'B' => 6,
                        'C' => 3,
                        c => panic!("{}", c),
                    }
                }
                c => panic!("{}", c),
            }
        })
        .sum()
}

pub fn b(input: Box<dyn Iterator<Item = String>>) -> i32 {
    input
        .map(|l| {
            let mut g = l.chars();
            let op = g.next();
            let op = op.unwrap();
            g.next();
            let me = g.next();
            let me = me.unwrap();
            match me {
                'X' => {
                    0 + match op {
                        'A' => 3,
                        'B' => 1,
                        'C' => 2,
                        c => panic!("{}", c),
                    }
                }
                'Y' => {
                    3 + match op {
                        'A' => 1,
                        'B' => 2,
                        'C' => 3,
                        c => panic!("{}", c),
                    }
                }
                'Z' => {
                    6 + match op {
                        'A' => 2,
                        'B' => 3,
                        'C' => 1,
                        c => panic!("{}", c),
                    }
                }
                c => panic!("{}", c),
            }
        })
        .sum()
}
