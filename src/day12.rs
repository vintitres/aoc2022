use std::collections::VecDeque;

use itertools::Itertools;

pub fn read(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .collect_vec()
        })
        .collect_vec()
}

fn findstart(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    unimplemented!();
}

pub fn part1(input: &str) -> usize {
    let map = read(input);
    let start = findstart(&map);
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let v = Vec::new();
    let moves:Vec<(i8,i8)>= vec![(0,1), (1,0), (0-1, 0), (0, 0-1)];
    loop {
        let ((x, y), s): ((usize, usize), i32) = q.pop_front().unwrap();
        let h = map[x][y];
        for (mx, my) in &moves {
            match map.get(x.checked_add(mx).unwrap_or_default()).unwrap_or(&v).get(y + my) {
                Some(hn) if (*hn == 'E' && (h == 'z' || h == 'y')) || *hn == h || *hn as u32 == h as u32 + 1 => q.push_back(((x + mx, y + my), s + 1)),
                _ => {}
            }
        }
    
    }
    1
}

pub fn part2(input: &str) -> usize {
    read(input);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day12.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1);
    }
}
