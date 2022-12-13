use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;

pub fn read(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn findstarts1(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return vec![(i, j)];
            }
        }
    }
    unimplemented!();
}

fn findstarts2(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' || map[i][j] == 'a' {
                ret.push((i, j));
            }
        }
    }
    ret
}

fn printm(map: &Vec<Vec<char>>) {
    for i in 1..map.len() {
        for j in 1..map[0].len() {
            print!("{}", map[i][j]);
        }
        println!("");
    }
}

pub fn dfs(input: &str, findstarts: fn(&Vec<Vec<char>>) -> Vec<(usize, usize)>) -> usize {
    let mut map = read(input);
    let starts = findstarts(&map);
    let mut q = VecDeque::new();
    for start in starts {
        q.push_back((start, 0));
    }
    let v = Vec::new();
    let moves: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0 - 1, 0), (0, 0 - 1)];
    loop {
        println!("{:?}", q.front());
        let ((x, y), s): ((usize, usize), usize) = q.pop_front().unwrap();
        let h = map[x][y];
        for (mx, my) in &moves {
            println!("m {:?} {:?}", mx, my);
            let nx = x as isize + *mx;
            if nx < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = y as isize + *my;
            if ny < 0 {
                continue;
            }
            let ny = ny as usize;
            println!("t {:?} {:?}", nx, ny);
            match map.get(nx).unwrap_or(&v).get(ny) {
                Some('E') => {
                    if h == 'z' || h == 'y' {
                        return s + 1;
                    }
                }
                Some(hn) if h == 'S' && (*hn == 'a' || *hn == 'b') => {
                    q.push_back(((nx, ny), s + 1))
                }
                Some(hn)
                    if hn.is_alphabetic() && h.is_alphabetic() && *hn as u32 <= h as u32 + 1 =>
                {
                    q.push_back(((nx, ny), s + 1))
                }
                _ => {}
            }
        }
        map[x][y] = '~';
    }
}

pub fn part1(input: &str) -> usize {
    dfs(input, findstarts1)
}

pub fn part2(input: &str) -> usize {
    dfs(input, findstarts2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day12.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 456);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 454);
    }
}
