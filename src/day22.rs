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

type StepFn = for<'a> fn(
    &'a [Vec<char>],
    (usize, usize),
    Facing,
    &[Wall],
) -> Option<((usize, usize), Facing)>;

fn walk(input: &str, step_fn: StepFn, walls: &[Wall]) -> u64 {
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
                    match step_fn(&map, pos, face, walls) {
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
    1000 * (pos.0 as u64 + 1)
        + 4 * (pos.1 as u64 + 1)
        + match face {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

pub fn part1(input: &str) -> u64 {
    walk(input, step_fn1, &[])
}

fn step_fn1(
    map: &[Vec<char>],
    pos: (usize, usize),
    face: Facing,
    _walls: &[Wall],
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

type LocalPos = (usize, usize);
type GlobalPos = (usize, usize);
struct WallConnectionInfo {
    wall_index: usize,
    new_facing: Facing,
    invert_shift: bool,
}
impl WallConnectionInfo {
    fn new_pos_when_entering(&self, shift: usize, walls: &[Wall]) -> (GlobalPos, Facing) {
        walls[self.wall_index].new_pos_when_entering(self.new_facing, shift, self.invert_shift)
    }
}

struct Wall {
    left_wall: WallConnectionInfo,
    right_wall: WallConnectionInfo,
    up_wall: WallConnectionInfo,
    down_wall: WallConnectionInfo,
    wall_start: GlobalPos,
}

impl Wall {
    fn local_pos(&self, pos: GlobalPos) -> LocalPos {
        (pos.0 - self.wall_start.0, pos.1 - self.wall_start.1)
    }
    fn global_pos(&self, pos: LocalPos) -> GlobalPos {
        (self.wall_start.0 + pos.0, self.wall_start.1 + pos.1)
    }
    fn new_pos_when_entering(
        &self,
        facing: Facing,
        shift: usize,
        invert_shift: bool,
    ) -> (GlobalPos, Facing) {
        (
            self.global_pos(match facing {
                Facing::Down => (0, if invert_shift { L - 1 - shift } else { shift }),
                Facing::Up => (L - 1, if invert_shift { L - 1 - shift } else { shift }),
                Facing::Right => (if invert_shift { L - 1 - shift } else { shift }, 0),
                Facing::Left => (if invert_shift { L - 1 - shift } else { shift }, L - 1),
            }),
            facing,
        )
    }

    fn try_step(&self, pos: GlobalPos, face: Facing, walls: &[Wall]) -> (GlobalPos, Facing) {
        let local_pos = self.local_pos(pos);
        match face {
            Facing::Up => {
                if local_pos.0 == 0 {
                    self.up_wall.new_pos_when_entering(local_pos.1, walls)
                } else {
                    ((pos.0 - 1, pos.1), face)
                }
            }
            Facing::Down => {
                if local_pos.0 == L - 1 {
                    self.down_wall.new_pos_when_entering(local_pos.1, walls)
                } else {
                    ((pos.0 + 1, pos.1), face)
                }
            }
            Facing::Left => {
                if local_pos.1 == 0 {
                    self.left_wall.new_pos_when_entering(local_pos.0, walls)
                } else {
                    ((pos.0, pos.1 - 1), face)
                }
            }
            Facing::Right => {
                if local_pos.1 == L - 1 {
                    self.right_wall.new_pos_when_entering(local_pos.0, walls)
                } else {
                    ((pos.0, pos.1 + 1), face)
                }
            }
        }
    }
    fn has_global_pos(&self, pos: GlobalPos) -> bool {
        pos.0 >= self.wall_start.0
            && pos.0 < self.wall_start.0 + L
            && pos.1 >= self.wall_start.1
            && pos.1 < self.wall_start.1 + L
    }
}

fn step_fn_cube(
    map: &[Vec<char>],
    pos: (usize, usize),
    face: Facing,
    walls: &[Wall],
) -> Option<((usize, usize), Facing)> {
    // eprintln!("{:?} {:?}", pos, face);
    let wall = walls.iter().find(|w| w.has_global_pos(pos)).unwrap();
    let (new_pos, new_face) = wall.try_step(pos, face, walls);
    if face != new_face {
        eprintln!("{:?} {:?} -> {:?} {:?}", pos, face, new_pos, new_face);
    }
    match map[new_pos.0].get(new_pos.1) {
        Some('#') => None,
        Some('.') => Some((new_pos, new_face)),
        _ => panic!("out of bounds: {:?}", new_pos),
    }
}

pub fn part2(input: &str) -> u64 {
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
    let _example_walls = vec![
        Wall {
            // mock wall to number form 1
            left_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            wall_start: (1000, 1000),
        },
        Wall {
            // 1
            left_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Left,
                invert_shift: true,
            },
            up_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Down,
                invert_shift: true,
            },
            down_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            wall_start: (0, 2 * L),
        },
        Wall {
            // 2
            left_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Up,
                invert_shift: true,
            },
            right_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Down,
                invert_shift: true,
            },
            down_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Up,
                invert_shift: true,
            },
            wall_start: (L, 0),
        },
        Wall {
            // 3
            left_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            wall_start: (L, L),
        },
        Wall {
            // 4
            left_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Down,
                invert_shift: true,
            },
            up_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            wall_start: (L, 2 * L),
        },
        Wall {
            // 5
            left_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Up,
                invert_shift: true,
            },
            right_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Up,
                invert_shift: true,
            },
            wall_start: (2 * L, 2 * L),
        },
        Wall {
            // 6
            left_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Left,
                invert_shift: true,
            },
            up_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Left,
                invert_shift: true,
            },
            down_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Right,
                invert_shift: true,
            },
            wall_start: (2 * L, 3 * L),
        },
    ];
    /*
       12
       3
      45
      6
    */
    let walls = vec![
        Wall {
            // mock wall to number form 1
            left_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 0,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            wall_start: (L * 10, L * 10),
        },
        Wall {
            // 1
            left_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Right,
                invert_shift: true,
            },
            right_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            wall_start: (0, L),
        },
        Wall {
            // 2
            left_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Left,
                invert_shift: true,
            },
            up_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            wall_start: (0, 2 * L),
        },
        Wall {
            // 3
            left_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            wall_start: (L, L),
        },
        Wall {
            // 4
            left_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Right,
                invert_shift: true,
            },
            right_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Right,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            wall_start: (2 * L, 0),
        },
        Wall {
            // 5
            left_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Left,
                invert_shift: true,
            },
            up_wall: WallConnectionInfo {
                wall_index: 3,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 6,
                new_facing: Facing::Left,
                invert_shift: false,
            },
            wall_start: (2 * L, L),
        },
        Wall {
            // 6
            left_wall: WallConnectionInfo {
                wall_index: 1,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            right_wall: WallConnectionInfo {
                wall_index: 5,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            up_wall: WallConnectionInfo {
                wall_index: 4,
                new_facing: Facing::Up,
                invert_shift: false,
            },
            down_wall: WallConnectionInfo {
                wall_index: 2,
                new_facing: Facing::Down,
                invert_shift: false,
            },
            wall_start: (3 * L, 0),
        },
    ];
    walk(input, step_fn_cube, &walls)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 197047);
    }
}
