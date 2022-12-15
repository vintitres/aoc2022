use std::collections::BTreeSet;

use itertools::Itertools;

type Pos = (i32, i32);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    closest_beacon: Pos,
}

impl Sensor {
    fn read(line: &str) -> Self {
        // println!("{:?}", line.split_ascii_whitespace().collect_vec());
        let (
            _sensor,
            _at,
            posx,
            posy,
            _closest,
            _beacon,
            _is,
            __at,
            closest_beacon_x,
            closest_beacon_y,
        ) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let pos = (
            posx[2..posx.len() - 1].parse().unwrap(),
            posy[2..posy.len() - 1].parse().unwrap(),
        );
        let closest_beacon = (
            closest_beacon_x[2..closest_beacon_x.len() - 1]
                .parse()
                .unwrap(),
            closest_beacon_y[2..].parse().unwrap(),
        );
        Sensor {
            pos,
            closest_beacon,
        }
    }
    fn closest_beacon_dist(&self) -> i32 {
        dist(self.pos, self.closest_beacon)
    }
    fn blocked_at_y(&self, y: i32) -> impl Iterator<Item = i32> {
        let bd = self.closest_beacon_dist();
        let yd = (self.pos.1 - y).abs();
        let fd = bd - yd;
        let l = self.pos.0 - fd;
        let r = self.pos.0 + fd;
        l..r
    }
}

fn dist(p1: Pos, p2: Pos) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1- p2.1).abs()
}

pub fn part1(input: &str) -> usize {
    let mut positions = BTreeSet::new();
    input.lines().map(Sensor::read).for_each(|sensor| {
        positions.extend(sensor.blocked_at_y(2000000));
    });
    positions.len()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day15.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4876693);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
