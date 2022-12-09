use std::collections::BTreeSet;

use itertools::Itertools;

type Mov = (i32, i32);
type Pos = (i32, i32);

pub fn read(input: &str) -> Vec<(Mov, usize)> {
    input
        .lines()
        .map(|line| {
            println!("{}", line);
            let (dir, cnt) = line.split(' ').collect_tuple().unwrap();
            (
                match dir.chars().next().unwrap() {
                    'D' => (0, -1),
                    'U' => (0, 1),
                    'R' => (1, 0),
                    'L' => (-1, 0),
                    _ => unimplemented!(),
                },
                cnt.parse::<usize>().unwrap(),
            )
        })
        .collect_vec()
}

fn follow(back: &mut Pos, front: &Pos) {
    let d0 = front.0 - back.0;
    let d1 = front.1 - back.1;
    match (d0.abs(), d1.abs()) {
        (0, 0) => {}
        (0, 1) => {}
        (1, 0) => {}
        (1, 1) => {}
        (2, 0) => back.0 += d0.signum(),
        (0, 2) => back.1 += d1.signum(),
        (1, 2) => {
            back.0 += d0.signum();
            back.1 += d1.signum();
        }
        (2, 1) => {
            back.0 += d0.signum();
            back.1 += d1.signum();
        }
        _ => unimplemented!(),
    }
}

pub fn part1(input: &str) -> usize {
    let moves = read(input);
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = BTreeSet::new();
    visited.insert(tail);
    for (mv, cnt) in moves {
        for _ in 0..cnt {
            head.0 += mv.0;
            head.1 += mv.1;
            follow(&mut tail, &head);
            visited.insert(tail);
        }
    }
    visited.len()
}

pub fn part2(input: &str) -> i32 {
    3
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day9.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 5907);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 3);
    }
}
