use itertools::Itertools;

type RobotCost = (i32, i32, i32);

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    minute: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
}

impl State {
    fn new() -> Self {
        Self {
            minute: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
    fn after(&self, time: i32) -> State {
        Self {
            minute: self.minute + time,
            ore: self.ore + time * self.ore_robots,
            clay: self.clay + time * self.clay_robots,
            obsidian: self.obsidian + time * self.obsidian_robots,
            geode: self.geode + time * self.geode_robots,
            ..*self
        }
    }

    fn _sub_cost(&self, cost: RobotCost) -> Self {
        State {
            ore: self.ore - cost.0,
            clay: self.clay - cost.1,
            obsidian: self.obsidian - cost.2,
            ..*self
        }
    }

    fn _add_ore_robot(&self) -> Self {
        Self {
            ore_robots: self.ore_robots + 1,
            ..*self
        }
    }

    fn _add_clay_robot(&self) -> Self {
        Self {
            clay_robots: self.clay_robots + 1,
            ..*self
        }
    }

    fn _add_obsidian_robot(&self) -> Self {
        Self {
            obsidian_robots: self.obsidian_robots + 1,
            ..*self
        }
    }

    fn _add_geode_robot(&self) -> Self {
        Self {
            geode_robots: self.geode_robots + 1,
            ..*self
        }
    }

    fn add_robot(&self, robot_type: &RobotType, blueprint: &Blueprint) -> Self {
        match robot_type {
            RobotType::Ore => self._sub_cost(blueprint.ore_robot_cost)._add_ore_robot(),
            RobotType::Clay => self._sub_cost(blueprint.clay_robot_cost)._add_clay_robot(),
            RobotType::Obsidian => self
                ._sub_cost(blueprint.obsidian_robot_cost)
                ._add_obsidian_robot(),
            RobotType::Geode => self
                ._sub_cost(blueprint.geode_robot_cost)
                ._add_geode_robot(),
        }
    }

    fn _time_until_can_build(&self, cost: RobotCost) -> Option<i32> {
        fn div_ceil(a: i32, b: i32) -> i32 {
            a / b + if a % b == 0 { 0 } else { 1 }
        }
        fn t(have: i32, cost: i32, prod: i32) -> Option<i32> {
            if prod == 0 && cost > 0 {
                None
            } else if have >= cost {
                Some(1)
            } else {
                Some(1 + div_ceil(cost - have, prod))
            }
        }
        let mut time = t(self.ore, cost.0, self.ore_robots).unwrap();
        match t(self.clay, cost.1, self.clay_robots) {
            None => return None,
            Some(clay_time) => time = core::cmp::max(time, clay_time),
        }
        match t(self.obsidian, cost.2, self.obsidian_robots) {
            None => return None,
            Some(obsidian_time) => time = core::cmp::max(time, obsidian_time),
        }
        Some(time)
    }

    fn time_until_can_build(&self, robot_type: &RobotType, blueprint: &Blueprint) -> Option<i32> {
        self._time_until_can_build(match robot_type {
            RobotType::Ore => blueprint.ore_robot_cost,
            RobotType::Clay => blueprint.clay_robot_cost,
            RobotType::Obsidian => blueprint.obsidian_robot_cost,
            RobotType::Geode => blueprint.geode_robot_cost,
        })
    }

    fn next(&self, robot_type: &RobotType, blueprint: &Blueprint, time_limit: i32) -> State {
        assert!(self.minute < time_limit);
        match self.time_until_can_build(robot_type, blueprint) {
            Some(time) if self.minute + time <= time_limit => {
                self.after(time).add_robot(robot_type, blueprint)
            }
            Some(_) => self.after(time_limit - self.minute),
            None => self.after(time_limit - self.minute),
        }
    }

    fn best(&self, blueprint: &Blueprint, time_limit: i32, cur_best: &mut i32) -> i32 {
        if self.minute == time_limit {
            return self.geode;
        }
        let time_left = time_limit - self.minute;
        if self.geode + time_left * self.geode_robots + time_left * time_left / 2 < *cur_best {
            return 0;
        }
        let mut b = 0;
        for robot_type in vec![
            RobotType::Ore,
            RobotType::Clay,
            RobotType::Obsidian,
            RobotType::Geode,
        ]
        .iter()
        .rev()
        {
            b = core::cmp::max(
                b,
                self.next(robot_type, blueprint, time_limit)
                    .best(blueprint, time_limit, cur_best),
            );
            *cur_best = b;
        }
        b
    }
}

struct Blueprint {
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

impl Blueprint {
    fn read(line: &str) -> Self {
        let (ore, clay, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = line
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .flat_map(|w| w.parse())
            .collect_tuple()
            .unwrap();
        Self {
            ore_robot_cost: (ore, 0, 0),
            clay_robot_cost: (clay, 0, 0),
            obsidian_robot_cost: (obsidian_ore, obsidian_clay, 0),
            geode_robot_cost: (geode_ore, 0, geode_obsidian),
        }
    }
    fn best(&self, time_limit: i32) -> i32 {
        State::new().best(self, time_limit, &mut 0)

        /*
        let mut b = 0;
        for last_ore_robot in 1..24 {
            for last_clay_robot in last_ore_robot..24 {
                for last_obsidian_robot in last_clay_robot..24 {
                    b = core::cmp::max(
                        b,
                        self.simulate(last_ore_robot, last_clay_robot, last_obsidian_robot),
                    );
                }
            }
        }
        b
        */
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(Blueprint::read)
        .enumerate()
        .map(|(i, b)| b.best(24) * (i as i32 + 1))
        .inspect(|s| println!("{:?}", s))
        .sum()
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .take(3)
        .map(Blueprint::read)
        .map(|b| b.best(32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day19.txt")
    }

    #[ignore = "slow"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1659);
    }

    #[ignore = "slow"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 6804);
    }
}
