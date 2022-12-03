use std::io::BufRead;

fn elves(input: impl BufRead) -> impl Iterator<Item = i32> {
    input
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
        .map(|l| l.unwrap())
}

pub fn a(input: impl BufRead) -> i32 {
    // elves(input).take(2).max().unwrap()
    elves(input).max().unwrap()
}

pub fn b(input: impl BufRead) -> i32 {
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
