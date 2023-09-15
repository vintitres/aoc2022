use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

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
        Some(Blizzard { x, y, dir })
    }
    fn blow(&self, max_x: usize, max_y: usize) -> Self {
        match self.dir {
            Direction::Left => {
                if self.y == 1 {
                    Blizzard { y: max_y, ..*self }
                } else {
                    Blizzard {
                        y: self.y - 1,
                        ..*self
                    }
                }
            }
            Direction::Right => {
                if self.y == max_y {
                    Blizzard { y: 1, ..*self }
                } else {
                    Blizzard {
                        y: self.y + 1,
                        ..*self
                    }
                }
            }
            Direction::Up => {
                if self.x == 1 {
                    Blizzard { x: max_x, ..*self }
                } else {
                    Blizzard {
                        x: self.x - 1,
                        ..*self
                    }
                }
            }
            Direction::Down => {
                if self.x == max_x {
                    Blizzard { x: 1, ..*self }
                } else {
                    Blizzard {
                        x: self.x + 1,
                        ..*self
                    }
                }
            }
        }
    }
}
pub fn part1(input: &str) -> usize {
    travel(input, 2)
}

fn travel(input: &str, start_action: usize) -> usize {
    let mut blizzards: HashSet<Blizzard> = HashSet::from_iter(
        input
            .lines()
            .enumerate()
            .flat_map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(y, c)| Blizzard::parse(x, y, c))
            })
            .flatten(),
    );
    let max_x = input.lines().collect_vec().len() - 2;
    let max_y = input.lines().next().unwrap().chars().collect_vec().len() - 2;

    blizzards = blow(&blizzards, max_x, max_y);
    let mut last_step = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let start_pos = (0, 1);
    let end_pos = (max_x + 1, max_y);
    let start_step = (0, start_pos, start_action);
    q.push_back(start_step);
    seen.insert(start_step);
    while !q.is_empty() {
        let (step, pos, action) = q.pop_front().unwrap();

        // eprintln!("{:?} {:?}", step, pos);
        // if step > 20 {
        //     break;
        // }

        if step == last_step + 1 {
            blizzards = blow(&blizzards, max_x, max_y);
            last_step = step;
        } else if step != last_step {
            panic!("unexpected step");
        }
        for new_pos in possible_moves_from(pos, max_x, max_y) {
            let mut next_step = (step + 1, new_pos, action);
            if seen.contains(&next_step) {
                continue;
            }
            // TODO switch to HashMap instead of trying all dirs  // ugh!
            if ![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .any(|dir| {
                blizzards.contains(&Blizzard {
                    x: new_pos.0,
                    y: new_pos.1,
                    dir: *dir,
                })
            }) {
                if action != 1 && new_pos == end_pos {
                    if action == 2 {
                        return step + 1;
                    } else if action == 0 {
                        next_step = (next_step.0, next_step.1, 1);
                    }
                } else if action == 1 && new_pos == start_pos {
                    next_step = (next_step.0, next_step.1, 2);
                }
                seen.insert(next_step);
                q.push_back(next_step);
            }
        }
    }
    panic!("No route found")
}

fn possible_moves_from(pos: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    if pos.0 == 0 {
        vec![
            pos,
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
        ]
    } else {
        vec![
            pos,
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
        ]
    }
    .iter()
    .filter(|(x, y)| {
        (*x == 0 && *y == 1)
            || (*x == max_x + 1 && *y == max_y)
            || (*x >= 1 && *x <= max_x && *y >= 1 && *y <= max_y)
    })
    .copied()
    .collect_vec()
}

fn blow(blizzards: &HashSet<Blizzard>, max_x: usize, max_y: usize) -> HashSet<Blizzard> {
    HashSet::from_iter(blizzards.iter().map(|b| b.blow(max_x, max_y)))
}

pub fn part2(input: &str) -> usize {
    travel(input, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day24.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 225);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 711);
    }
}
