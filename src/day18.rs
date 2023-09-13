use itertools::Itertools;
use std::collections::HashSet;

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

pub fn part1(input: &str) -> i64 {
    let cubes: HashSet<Cube> = input.lines().map(Cube::read).collect();
    let cubes2 = cubes.clone();
    let mut surface = 0;
    for cube in cubes {
        for side in vec![
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ] {
            let (sx, sy, sz) = side;
            if !cubes2.contains(&Cube {
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

pub fn part2(input: &str) -> usize {
    input.len()
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
        assert_eq!(part2(input()), 1234);
    }
}
