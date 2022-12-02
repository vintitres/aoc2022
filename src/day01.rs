use std::io::BufRead;

// TODO do I have to Box?
fn elves(input: Box<dyn BufRead>) -> Box<dyn Iterator<Item = i32>> {
    // TODO shorter init?
    let mut v = Vec::new();
    v.push(Vec::new());

    Box::new(
        input
            .lines()
            .map(|l| l.unwrap())
            .fold(v, |mut acc, line| {
                if line.is_empty() {
                    acc.push(Vec::new());
                } else {
                    acc.last_mut().unwrap().push(line);
                }
                acc
            })
            // TODO
            // .split(|l: String| l.is_empty());
            .into_iter()
            .map(|e| e.into_iter().map(|f| f.parse::<i32>().unwrap()))
            .map(|x| x.sum::<i32>()),
    )
}

pub fn a(input: Box<dyn BufRead>) -> i32 {
    elves(input).max().unwrap()
}

pub fn b(input: Box<dyn BufRead>) -> i32 {
    // TODO shorter init?
    let mut v = Vec::new();
    v.push(0);
    v.push(0);
    v.push(0);

    elves(input)
        .fold(v, |mut top3, x| {
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
