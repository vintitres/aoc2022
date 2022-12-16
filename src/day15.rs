use itertools::Itertools;

type Pos = (i64, i64);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    closest_beacon_dist: i64,
}

impl Sensor {
    fn read(line: &str) -> Self {
        let (_, _, sx, sy, _, _, _, _, bx, by) =
            line.split_ascii_whitespace().collect_tuple().unwrap();
        let pos = (
            sx[2..sx.len() - 1].parse().unwrap(),
            sy[2..sy.len() - 1].parse().unwrap(),
        );
        let closest_beacon = (
            bx[2..bx.len() - 1].parse().unwrap(),
            by[2..].parse().unwrap(),
        );
        Sensor {
            pos,
            closest_beacon_dist: dist(closest_beacon, pos),
        }
    }
    fn blocked_at_y(&self, y: i64) -> (i64, i64) {
        let bd = self.closest_beacon_dist;
        let yd = (self.pos.1 - y).abs();
        let fd = bd - yd;
        let l = self.pos.0 - fd;
        let r = self.pos.0 + fd;
        (l, r)
    }

    fn _border(xl: i64, xr: i64, ys: i64, ye: i64) -> impl Iterator<Item = Pos> {
        let mut skip = 0;
        if xl < 0 {
            skip = xl.abs();
        }
        let yrange: Box<dyn Iterator<Item = i64>>;
        if ys < ye {
            if ys < 0 {
                skip = core::cmp::max(skip, ys.abs());
            }
            yrange = Box::new(ys + skip..ye);
        } else {
            if ye < 0 {
                skip = core::cmp::max(skip, ye.abs());
            }
            yrange = Box::new((ye + skip..ys).rev());
        }
        (xl + skip..xr)
            .take_while(|x| *x <= 4000000)
            .zip(yrange.take_while(|y| *y <= 4000000))
    }

    fn border(&self) -> impl Iterator<Item = Pos> {
        let bd = self.closest_beacon_dist + 1;
        let lt = Self::_border(self.pos.0 - bd, self.pos.0, self.pos.1, self.pos.1 - bd);
        let rt = Self::_border(self.pos.0, self.pos.0 + bd, self.pos.1 - bd, self.pos.1);
        let lb = Self::_border(self.pos.0 - bd, self.pos.0, self.pos.1, self.pos.1 + bd);
        let rb = Self::_border(self.pos.0, self.pos.0 + bd, self.pos.1 + bd, self.pos.1);

        lt.chain(rt).chain(lb).chain(rb) //.inspect(|(x,y)| assert!(*x >= 0 && *x <= 4000000 && *y >= 0 && *y <= 4000000))
    }
    fn in_range(&self, p: Pos) -> bool {
        dist(self.pos, p) <= self.closest_beacon_dist
    }
}

fn dist(p1: Pos, p2: Pos) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

pub fn part1(input: &str) -> i64 {
    let intervals = input
        .lines()
        .map(Sensor::read)
        .map(|sensor| sensor.blocked_at_y(2000000))
        .sorted()
        .collect_vec();
    let mut res = intervals[0].1 - intervals[0].0;
    let mut lastintervalend = intervals[0].1;
    for interval in &intervals[1..] {
        if interval.1 < lastintervalend {
            continue;
        } else if interval.0 < lastintervalend {
            res += interval.1 - lastintervalend;
        } else {
            res += interval.1 - interval.0;
        }
        lastintervalend = interval.1;
    }
    res
}

fn _limit(x: i64) -> i64 {
    core::cmp::min(core::cmp::max(x, 0), 4000000)
}
fn limit((b, e): (i64, i64)) -> (i64, i64) {
    (_limit(b), _limit(e))
}

pub fn part2(input: &str) -> i64 {
    0
}
pub fn part2_slow(input: &str) -> i64 {
    let sensors = input.lines().map(Sensor::read).collect_vec();
    for y in 0..=4000000 {
        let mut lastend = 0;
        let intervals = sensors
            .iter()
            .map(|sensor| sensor.blocked_at_y(y))
            .map(limit)
            .filter(|(b, e)| b != e)
            .sorted();
        for (b, e) in intervals {
            if b > lastend {
                return 4000000 * (lastend + 1) + y;
            } else {
                lastend = core::cmp::max(lastend, e);
            }
        }
    }
    unimplemented!();
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
        assert_eq!(part2(input()), 11645454855041);
    }
}
