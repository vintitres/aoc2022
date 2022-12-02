use std::io::BufRead;

// TODO do I have to Box?
fn elves(input: Box<dyn BufRead>) -> Box<dyn Iterator<Item = i32>> {
    let e = input
        .lines()
        .map(|l| {
            // println!("{:?}", l);
            l.unwrap()
        })
        .scan(0, |last_elf, line| {
            if line.is_empty() {
                let full_elf = *last_elf;
                *last_elf = 0;
                Some(Some(full_elf))
            } else {
                *last_elf += line.parse::<i32>().unwrap();
                Some(None)
            }
        })
        .filter(|e| e.is_some())
        .map(|l| l.unwrap());
    Box::new(e)
}

pub fn a(input: Box<dyn BufRead>) -> i32 {
    // elves(input).take(2).max().unwrap()
    elves(input).max().unwrap()
}

pub fn b(input: Box<dyn BufRead>) -> i32 {
    elves(input)
        .fold(vec![0, 0, 0], |mut top3, x| {
            let mut x = x;
            for i in 0..3 {
                if x > top3[i] {
                    let xx = top3[i];
                    top3[i] = x;
                    x = xx;
                }
            }
            top3
        })
        .into_iter()
        .sum()
}
