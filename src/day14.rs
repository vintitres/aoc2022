use std::collections::BTreeSet;

use itertools::Itertools;

struct RockPath {
    points: Vec<(usize, usize)>,
}

fn between(a: usize, b: usize) -> impl Iterator<Item = usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }

}

impl RockPath {
    fn read(input: &str) -> Self {
        RockPath {points: input.split(" -> ").map(|point| point.split(',').map(|c| c.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>().unwrap()).collect_vec()}
    }
    fn allpoints(&self) -> BTreeSet<(usize, usize)> {
        let mut allpoints = BTreeSet::new();
        let mut lastpoint = self.points.first().unwrap();
        for p in &self.points[1..] {
            if lastpoint.0 == p.0 {
                for y in between(lastpoint.1, p.1) {
                    allpoints.insert((p.0, y));
                }
            } else {
                for x in between(lastpoint.0, p.0) {
                    allpoints.insert((x, p.1));
                }
            }
            lastpoint = p;
        }
        allpoints
    }
}

pub fn part1(input: &str) -> usize {
    dropsand(input, false)
}
fn dropsand(input: &str, floor: bool) -> usize {
    let mut rockandsand = BTreeSet::from_iter(input.lines().map(RockPath::read).flat_map(|p| {
        p.allpoints()
    }));
    let maxy = rockandsand.clone();
    let maxy = maxy.iter().map(|(_x, y)| y).max().unwrap();
    const SANDSTART: (usize, usize) = (500,0);
    let mut lastsand = SANDSTART;
    let mut sandcount = 0;
    loop {
        let (x, y) = lastsand;
        //print(&rockandsand, lastsand);
        let mut moved = false;
        for p in &[(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
            if !rockandsand.contains(p) {
                lastsand = *p;
                moved = true;
                break;
            }
        }
        if !moved || (floor && lastsand.1 == maxy + 1) {
            rockandsand.insert(lastsand);
            sandcount += 1;
            if lastsand == SANDSTART {
                break;
            }
            lastsand = SANDSTART;
        }
        if y >= 1000 {
            break;
        }
    }
    sandcount
}

pub fn print(rockandsand: &BTreeSet<(usize,usize)>, falling: (usize, usize)) {
    for y in 0..30 {
        for x in 490..510 {
            print!("{}", if (x, y) == falling { '~' } else if rockandsand.contains(&(x,y)) { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

pub fn part2(input: &str) -> usize {
    dropsand(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day14.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1072);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 24659);
    }
}
