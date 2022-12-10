use itertools::Itertools;

#[derive(Debug)]
enum Op {
    Noop,
    Addx(i32),
}

fn readline(line: &str) -> Op {
    match line {
        "noop" => Op::Noop,
        line => Op::Addx(line.split(' ').nth(1).unwrap().parse().unwrap()),
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(readline)
        .scan((1, 0), |(x, cycle), op| {
            let mut ret = None;
            match op {
                Op::Noop => {
                    *cycle += 1;
                }
                Op::Addx(v) => {
                    if (*cycle + 1) % 40 == 20 {
                        ret = Some(*x * (*cycle + 1))
                    }
                    *cycle += 2;
                    *x += v;
                }
            }
            if *cycle % 40 == 20 {
                ret = Some(*x * *cycle)
            }
            Some(ret)
        })
        .flatten()
        .sum()
}

fn draw(cycle: i32, x: i32) -> String {
    let mut ret = "".to_string();
    if (cycle % 40 - x as i32).abs() <= 1 {
        ret += "#";
    } else {
        ret += ".";
    }
    if cycle % 40 == 39 {
        ret += "\n";
    }
    ret
}

pub fn part2(input: &str) -> String {
    let crt = input
        .lines()
        .map(readline)
        .scan((1, 0), |(x, cycle), op| {
            let mut ret = draw(*cycle, *x);
            match op {
                Op::Noop => {
                    *cycle += 1;
                }
                Op::Addx(v) => {
                    ret.push_str(&draw(*cycle + 1, *x));
                    *cycle += 2;
                    *x += v;
                }
            }
            Some(ret)
        })
        .collect_vec()
        .join("");
    print!("{}", crt);
    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day10.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 15120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(input()),
            include_str!("../input/2022/day10part2out.txt")
        );
    }
}
