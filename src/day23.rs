use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::repeat;

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}
const DIRECTIONS: [Direction; 4] = [Direction::N, Direction::S, Direction::W, Direction::E];

const ELF: char = '#';

pub fn part1(input: &str) -> isize {
    let mut elves: HashSet<Elf> = HashSet::from_iter(
        input
            .lines()
            .enumerate()
            .map(|(x, l)| {
                l.chars().enumerate().map(move |(y, c)| {
                    if c == ELF {
                        Some(Elf {
                            x: x as isize,
                            y: y as isize,
                        })
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .flatten(),
    );
    for step in 0..10 {
        let maybe_new_elfs = elves.iter().map(|e| e.maybe_move(step, &elves));
        let mut count_map = HashMap::new();
        maybe_new_elfs.for_each(|e| *count_map.entry(e).or_insert(0) += 1);
        let allowed_moves = HashSet::from_iter(count_map.iter().filter_map(|(elf, &count)| {
            if count == 1 {
                Some(*elf)
            } else {
                None
            }
        }));
        let new_elves = HashSet::from_iter(
            elves
                .iter()
                .map(|e| e.move_if_allowed(step, &elves, &allowed_moves)),
        );
        draw_field(&elves, &new_elves);
        elves = new_elves;
    }
    let minmax_x = elves.iter().map(|e| e.x).minmax();
    let minmax_y = elves.iter().map(|e| e.y).minmax();
    if let (MinMax(min_x, max_x), MinMax(min_y, max_y)) = (minmax_x, minmax_y) {
        (max_x - min_x + 1) * (max_y - min_y + 1) - (elves.len() as isize)
    } else {
        panic!("unexpected field size");
    }
}

fn draw_field(elves: &Elfs, new_elves: &Elfs) {
    let minmax_x = elves.iter().map(|e| e.x).minmax();
    let minmax_y = elves.iter().map(|e| e.y).minmax();
    if let (MinMax(min_x, max_x), MinMax(min_y, max_y)) = (minmax_x, minmax_y) {
        for x in irange(min_x, max_x) {
            for y in irange(min_y, max_y) {
                eprint!(
                    "{:}",
                    if elves.contains(&Elf { x, y }) {
                        "#"
                    } else if new_elves.contains(&Elf { x, y }) {
                        "@"
                    } else {
                        "."
                    }
                );
            }
            eprintln!();
        }
        eprintln!();
    } else {
        panic!("unexpected field size");
    }
}

type Elfs = HashSet<Elf>;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct Elf {
    x: isize,
    y: isize,
}

fn irange(l: isize, r: isize) -> impl Iterator<Item = isize> {
    (0..=((r - l) as usize)).map(move |i| i as isize + l)
}

impl Elf {
    fn dir_elves(&self, direction: &Direction) -> Vec<Elf> {
        let (x, y) = (self.x, self.y);
        match direction {
            Direction::E => irange(x - 1, x + 1)
                .zip(repeat(y + 1))
                .map(|(x, y)| Elf { x, y })
                .collect_vec(),
            Direction::W => irange(x - 1, x + 1)
                .zip(repeat(y - 1))
                .map(|(x, y)| Elf { x, y })
                .collect_vec(),
            Direction::N => repeat(x - 1)
                .zip(irange(y - 1, y + 1))
                .map(|(x, y)| Elf { x, y })
                .collect_vec(),
            Direction::S => repeat(x + 1)
                .zip(irange(y - 1, y + 1))
                .map(|(x, y)| Elf { x, y })
                .collect_vec(),
        }
    }
    fn dir_step(&self, direction: &Direction) -> Elf {
        let (x, y) = (self.x, self.y);
        match direction {
            Direction::E => Elf { x, y: y + 1 },
            Direction::W => Elf { x, y: y - 1 },
            Direction::S => Elf { x: x + 1, y },
            Direction::N => Elf { x: x - 1, y },
        }
    }
    fn try_go(&self, direction: &Direction, elves: &Elfs) -> Option<Elf> {
        if self.dir_elves(direction).iter().any(|e| elves.contains(e)) {
            None
        } else {
            Some(self.dir_step(direction))
        }
    }

    fn elves_around(&self) -> impl Iterator<Item = Elf> + '_ {
        irange(self.x - 1, self.x + 1)
            .map(move |x| {
                irange(self.y - 1, self.y + 1).map(move |y| {
                    if x == self.x && y == self.y {
                        None
                    } else {
                        Some(Elf { x, y })
                    }
                })
            })
            .flatten()
            .flatten()
    }

    fn maybe_move(&self, step: usize, elves: &Elfs) -> Elf {
        if self.x == 0 && self.y == 4 {
            eprintln!("{:?}", self.elves_around().collect_vec());
        }
        if !self.elves_around().any(|e| elves.contains(&e)) {
            return Elf {
                x: self.x,
                y: self.y,
            };
        }
        for direction in (0..(DIRECTIONS.len())).map(|i| DIRECTIONS[(step + i) % DIRECTIONS.len()])
        {
            if let Some(elf) = self.try_go(&direction, elves) {
                return elf;
            }
        }
        Elf {
            x: self.x,
            y: self.y,
        }
    }
    fn move_if_allowed(&self, step: usize, elves: &Elfs, allowed_moves: &HashSet<Elf>) -> Elf {
        let maybe_new_elf = self.maybe_move(step, elves);
        if allowed_moves.contains(&maybe_new_elf) {
            maybe_new_elf
        } else {
            Elf {
                x: self.x,
                y: self.y,
            }
        }
    }
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day23.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 3871);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
