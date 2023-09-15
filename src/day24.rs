use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Blizzard {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Blizzard {
    fn parse(x: usize, y: usize, c: char) -> Option<Self> {
        let dir = match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => return None,
        };
        Some(Blizzard {x,y,dir})
    }
    fn blow(&self, max_x: usize, max_y: usize) -> Self {
        match self.dir {
            Direction::Left => if self.y == 1 {Blizzard { y: max_y, ..*self }} else { Blizzard{y: self.y - 1, ..*self } }
            Direction::Right => if self.y == max_y {Blizzard { y: 1, ..*self }} else { Blizzard{y: self.y + 1, ..*self } }
            Direction::Up => if self.x == 1 {Blizzard { x: max_x, ..*self }} else { Blizzard{x: self.x - 1, ..*self } }
            Direction::Down => if self.x == max_x {Blizzard { x: 1, ..*self }} else { Blizzard{x: self.x + 1, ..*self } }

        }
    }
}
pub fn part1(input: &str) -> usize {
    let mut blizzards: HashSet<Blizzard> = HashSet::from_iter(
        input
            .lines()
            .enumerate()
            .flat_map(|(x, l)| {
                l.chars().enumerate().map(move |(y, c)| {
                    Blizzard::parse(x, y, c)
                })
            })
            .flatten());
    let max_x = input.lines().collect_vec().len() - 1;
    let max_y = input.lines().next().unwrap().chars().collect_vec().len() - 1;
    
    blizzards = blow(&blizzards, max_x, max_y);
    let mut last_step = 0;
    let mut q = VecDeque::new();
    q.push_back((0, (max_x + 1, max_y)));
    while !q.is_empty() {
        let (step, pos) = q.pop_front().unwrap();
        if step == last_step + 1 {
            blizzards = blow(&blizzards, max_x, max_y);
            last_step = step;
        } else if step != last_step {
            panic!("unexpected step");
        }
        
    }
    1
}

fn blow(blizzards: &HashSet<Blizzard>, max_x: usize, max_y: usize) -> HashSet<Blizzard> {
    HashSet::from_iter(blizzards.iter().map(|b| b.blow(max_x, max_y)))
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day24.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
