use std::io::BufRead;

pub fn a(input: Box<dyn BufRead>) -> i32 {
    input
        .lines()
        .map(|l| {
            let g = l.unwrap();
            let mut g = g.chars();
            let op = g.next();
            let op = op.unwrap();
            g.next();
            let me = g.next();
            let me = me.unwrap();
            match me {
                'X' => {
                    1 +
                    match op {
                        'A' => 3,
                        'B' => 0,
                        'C' => 6,
                        c => panic!("{}", c),
                    }
                },
                'Y' => {
                    2 +
                    match op {
                        'A' => 6,
                        'B' => 3,
                        'C' => 0,
                        c => panic!("{}", c),
                    }
                },
                'Z' =>  {
                    3 +
                    match op {
                        'A' => 0,
                        'B' => 6,
                        'C' => 3,
                        c => panic!("{}", c),
                    }
                },
                c => panic!("{}", c),
            }
        }).sum()
}
