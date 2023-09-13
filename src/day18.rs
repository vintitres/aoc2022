use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

const SIDES: [(i32, i32, i32); 6] = [
    (0, 0, 1),
    (0, 0, -1),
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
];

const LEAKS: [(i32, i32, i32); 20] = [
    (1, 1, 0),
    (1, 0, 1),
    (1, -1, 0),
    (1, 0, -1),
    (1, 1, 1),
    (1, 1, -1),
    (1, -1, 1),
    (1, -1, -1),
    (-1, 1, 0),
    (-1, 0, 1),
    (-1, -1, 0),
    (-1, 0, -1),
    (-1, 1, 1),
    (-1, 1, -1),
    (-1, -1, 1),
    (-1, -1, -1),
    (0, 1, 1),
    (0, 1, -1),
    (0, -1, 1),
    (0, -1, -1),
];

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}
impl Cube {
    fn read(line: &str) -> Self {
        let (x, y, z) = line.splitn(3, ',').collect_tuple().unwrap();
        Cube {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let cubes: HashSet<Cube> = input.lines().map(Cube::read).collect();
    let mut surface = 0;
    for cube in &cubes {
        for side in SIDES {
            let (sx, sy, sz) = side;
            if !cubes.contains(&Cube {
                x: cube.x + sx,
                y: cube.y + sy,
                z: cube.z + sz,
            }) {
                surface += 1;
            }
        }
    }
    surface
}

pub fn part2(input: &str) -> i32 {
    let cubes: HashSet<Cube> = input.lines().map(Cube::read).collect();
    let max_x = cubes.iter().map(|c| c.x).max().unwrap() + 5;
    let max_y = cubes.iter().map(|c| c.y).max().unwrap() + 5;
    let max_z = cubes.iter().map(|c| c.z).max().unwrap() + 5;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let start = (0, 0, 0);
    q.push_back(start);
    seen.insert(start);
    let mut surface = 0;
    while !q.is_empty() {
        let (px, py, pz) = q.pop_front().unwrap();
        for side in SIDES {
            let (sx, sy, sz) = side;
            let xyz = (px + sx, py + sy, pz + sz);
            let (x, y, z) = xyz;
            if x >= 0 && x <= max_x && y >= 0 && y <= max_y && z >= 0 && z <= max_z {
                if cubes.contains(&Cube { x, y, z }) {
                    surface += 1;
                } else if !seen.contains(&xyz) {
                    q.push_back(xyz);
                    seen.insert(xyz);
                }
            }
        }
        for leak in LEAKS {
            let (sx, sy, sz) = leak;
            let xyz = (px + sx, py + sy, pz + sz);
            let (x, y, z) = xyz;
            if x >= 0 && x <= max_x && y >= 0 && y <= max_y && z >= 0 && z <= max_z {
                if !cubes.contains(&Cube { x, y, z }) && !seen.contains(&xyz) {
                    q.push_back(xyz);
                    seen.insert(xyz);
                }
            }
        }
    }
    surface
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day18.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 3326);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 3318);
    }
}
