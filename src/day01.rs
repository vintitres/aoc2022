use std::io::BufRead;

// TODO do I have to Box?
fn elves(input: Box<dyn BufRead>) -> Box<dyn Iterator<Item = i32>> {
    Box::new(
        input
            .lines()
            .map(|l| l.unwrap())
            .fold((vec![], 0), |(mut elves, mut last_elf), line| {
                if line.is_empty() {
                    elves.push(last_elf);
                    last_elf = 0;
                } else {
                    last_elf += line.parse::<i32>().unwrap();
                }
                (elves, last_elf)
            })
            .0
            .into_iter(),
    )
}

pub fn a(input: Box<dyn BufRead>) -> i32 {
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
