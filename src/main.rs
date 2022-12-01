use std::io::BufRead;

fn main() {
    let mut v = Vec::new();
    v.push(Vec::new());
    let r = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .fold(v, |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                acc.last_mut().unwrap().push(line.parse::<i32>().unwrap());
            }
            acc
        })
        .into_iter()
        .map(|x| x.into_iter().sum::<i32>());
    // let r = r.max().unwrap();
    let mut v = Vec::new();
    v.push(0);
    v.push(0);
    v.push(0);
    let r: i32 = r
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
        .sum();
    println!("{:?}", r);
}
