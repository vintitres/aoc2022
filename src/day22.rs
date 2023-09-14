use itertools::Itertools;

const L: usize = 50;

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
    step_fn: for<'a> fn(
        &'a Vec<Vec<char>>,
        (usize, usize),
        Facing,
    ) -> Option<((usize, usize), Facing)>,
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
                        Some((p, f)) => {
                            pos = p;
                            face = f;
                        }
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

fn step_fn1(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    face: Facing,
) -> Option<((usize, usize), Facing)> {
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
            Some('.') => return Some((new_pos, face)),
            _ => pos = new_pos,
        }
    }
}

fn step_fn_cube(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    face: Facing,
) -> Option<((usize, usize), Facing)> {
    let (new_pos, new_face) = match face {
        /*
              12-----|
              |     1111
              |     1111
              |  13-1111------16
              |  |  1111       |
            222233334444       |
            222233334444-46    |
        |---222233334444  |    |
        |   222233334444  |    |
        |     |  |  55556666   |
        |     |  35-55556666---|
        |     |     55556666
        |     |     55556666
        |     25-----|    |
        26----------------|


        */
        Facing::Down => {
            if pos.0 == 2 * L - 1 && pos.1 < L {
                // 25
                ((3 * L - 1, 2 * L + L - 1 - pos.1), Facing::Up)
            } else if pos.0 == 2 * L - 1 && pos.1 >= L && pos.1 < 2 * L {
                // 35
                ((2 * L + 2 * L - 1 - pos.1, 2 * L), Facing::Right)
            } else if pos.0 == 3 * L && pos.1 >= 2 * L && pos.1 < 3 * L {
                // 52
                ((2 * L, 3 * L - 1 - pos.1), Facing::Up)
            } else if pos.0 == 3 * L && pos.1 >= 3 * L {
                // 62
                ((L + 4 * L - 1 - pos.1, 0), Facing::Right)
            } else {
                ((pos.0 + 1, pos.1), face)
            }
        }
        Facing::Up => {
            if pos.1 < L && pos.0 == L - 1 {
                // 21
                ((0, 2 * L + L - 1 - pos.1), Facing::Down)
            } else if pos.1 >= L && pos.1 < 2 * L && pos.0 == L - 1 {
                // 31
                ((pos.1 - L, 2 * L), Facing::Right)
            } else if pos.0 == 0 && pos.1 >= 2 * L && pos.1 < 3 * L {
                // 12
                ((L, 3 * L - 1 + pos.1), Facing::Down)
            } else if pos.0 == 2 * L && pos.1 >= 3 * L {
                // 64
                ((L + 4 * L - 1 - pos.1, 3 * L - 1), Facing::Left)
            } else {
                ((pos.0 - 1, pos.1), face)
            }
        }
        Facing::Left => {
            if pos.0 < L && pos.1 == 2 * L {
                // 13
                ((L, L + pos.0), Facing::Down)
            } else if pos.0 >= L && pos.0 < 2 * L && pos.1 == 0 {
                // 26
                ((3 * L - 1, 3 * L + 2 * L - 1 - pos.0), Facing::Up)
            } else if pos.0 >= 2 * L && pos.1 == 2 * L {
                // 53
                ((3 * L - 1, L + 3 * L - 1 - pos.0), Facing::Up)
            } else {
                ((pos.0, pos.1 - 1), face)
            }
        }
        Facing::Right => {
            if pos.0 < L && pos.1 == 3 * L - 1 {
                // 16
                ((2 * L + L - 1 - pos.0, 4 * L - 1), Facing::Left)
            } else if pos.0 >= L && pos.0 < 2 * L && pos.1 == 3 * L - 1 {
                // 46
                ((2 * L, 3 * L + 2 * L - 1 - pos.0), Facing::Down)
            } else if pos.0 >= 2 * L && pos.1 == 4 * L - 1 {
                // 61
                ((3 * L - 1 - pos.0, 3 * L - 1), Facing::Left)
            } else {
                ((pos.0, pos.1 + 1), face)
            }
        }
    };
    match map[new_pos.0].get(new_pos.1) {
        Some('#') => None,
        Some('.') => Some((new_pos, new_face)),
        _ => panic!("out of bounds: {:?}", pos),
    }
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
