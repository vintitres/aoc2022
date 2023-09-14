use itertools::Itertools;

#[derive(Debug)]
enum Wise {
    Counterclockwise,
    Clockwise,
}

#[derive(Debug)]
enum Move {
    Walk(usize),
    Turn(Wise),
}

#[derive(Clone, PartialEq, Debug, Copy)]
enum Facing {
    Left,
    Up,
    Right,
    Down,
}
impl Facing {
    fn turn(&self, direction: Wise) -> Self {
        let values = [Self::Left, Self::Up, Self::Right, Self::Down].iter();
        match direction {
            Wise::Clockwise => *values.cycle().skip_while(|&f| f != self).nth(1).unwrap(),
            Wise::Counterclockwise => *values
                .rev()
                .cycle()
                .skip_while(|&f| f != self)
                .nth(1)
                .unwrap(),
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut elements = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<usize>() {
                    elements.push(Move::Walk(num));
                } else {
                    eprintln!("Failed to parse number: {}", current_number);
                }
                current_number.clear();
            }
            elements.push(match c {
                'L' => Move::Turn(Wise::Counterclockwise),
                'R' => Move::Turn(Wise::Clockwise),
                _ => unimplemented!(),
            });
        }
    }

    // Handle the case when the string ends with a number
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<usize>() {
            elements.push(Move::Walk(num));
        } else {
            eprintln!("Failed to parse number: {}", current_number);
        }
    }
    elements
}

fn walk(
    input: &str,
    step_fn: for<'a> fn(&'a Vec<Vec<char>>, (usize, usize), Facing) -> Option<(usize, usize)>,
) -> usize {
    let input = input.lines().collect_vec();
    let (map, moves) = input.split_at(input.len() - 2);
    let map = map.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let moves = parse_moves(moves[1]);
    let mut face = Facing::Right;
    let mut pos = (0, map[0].iter().position(|&c| c == '.').unwrap());
    for m in moves {
        match m {
            Move::Walk(steps) => {
                for _ in 0..steps {
                    match step_fn(&map, pos, face) {
                        None => break,
                        Some(p) => pos = p,
                    }
                }
            }
            Move::Turn(wise) => {
                face = face.turn(wise);
            }
        }
    }
    1000 * (pos.0 + 1)
        + 4 * (pos.1 + 1)
        + match face {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

pub fn part1(input: &str) -> usize {
    walk(input, step_fn1)
}

fn step_fn1(map: &Vec<Vec<char>>, pos: (usize, usize), face: Facing) -> Option<(usize, usize)> {
    let mut pos = pos;
    loop {
        let new_pos = match face {
            Facing::Down => (if pos.0 + 1 == map.len() { 0 } else { pos.0 + 1 }, pos.1),
            Facing::Up => (if pos.0 == 0 { map.len() - 1 } else { pos.0 - 1 }, pos.1),
            Facing::Left => (
                pos.0,
                if pos.1 == 0 {
                    map[pos.0].len() - 1
                } else {
                    pos.1 - 1
                },
            ),
            Facing::Right => (
                pos.0,
                if pos.1 + 1 == map[pos.0].len() {
                    0
                } else {
                    pos.1 + 1
                },
            ),
        };
        match map[new_pos.0].get(new_pos.1) {
            Some('#') => return None,
            Some('.') => return Some(new_pos),
            _ => pos = new_pos,
        }
    }
}

fn step_fn_cube(map: &Vec<Vec<char>>, pos: (usize, usize), face: Facing) -> Option<(usize, usize)> {
    Some(pos)
}

pub fn part2(input: &str) -> usize {
    walk(input, step_fn_cube)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day22.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 56372);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
