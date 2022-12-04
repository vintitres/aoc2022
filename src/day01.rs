use aoc_runner_derive::aoc;

fn elves(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .scan(0, |last_elf, line| {
            let line = line;
            // println!("{:?}", l);
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

#[aoc(day1, part1)]
fn a(input: &str) -> i32 {
    // elves(input).take(2).max().unwrap()
    elves(input).max().unwrap()
}

#[aoc(day1, part2)]
fn b(input: &str) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day1.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(a(input()), 67658);
    }

    #[test]
    fn test_part2() {
        assert_eq!(b(input()), 200158);
    }
}
