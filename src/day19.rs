use itertools::Itertools;

type RobotCost = (i32, i32, i32);

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
#[derive(Debug)]
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
    fn produce_and_try_build(&mut self, robot_type: RobotType, blueprint: &Blueprint) {
        let built_robot = match robot_type {
            RobotType::Ore => match self.ore - blueprint.ore_robot_cost.0 {
                ore if ore >= 0 => {
                    self.ore = ore;
                    true
                }
                _ => false,
            },
            RobotType::Clay => match self.ore - blueprint.clay_robot_cost.0 {
                ore if ore >= 0 => {
                    self.ore = ore;
                    true
                }
                _ => false,
            },
            RobotType::Obsidian => match (
                self.ore - blueprint.obsidian_robot_cost.0,
                self.clay - blueprint.obsidian_robot_cost.1,
            ) {
                (ore, clay) if ore >= 0 && clay >= 0 => {
                    self.ore = ore;
                    self.clay = clay;
                    true
                }
                _ => false,
            },
            RobotType::Geode => match (
                self.ore - blueprint.geode_robot_cost.0,
                self.obsidian - blueprint.geode_robot_cost.2,
            ) {
                (ore, obsidian) if ore >= 0 && obsidian >= 0 => {
                    self.ore = ore;
                    self.obsidian = obsidian;
                    true
                }
                _ => false,
            },
        };
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        if built_robot {
            match robot_type {
                RobotType::Ore => self.ore_robots += 1,
                RobotType::Clay => self.clay_robots += 1,
                RobotType::Obsidian => self.obsidian_robots += 1,
                RobotType::Geode => self.geode_robots += 1,
            }
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
        Self { ore_robots: self.ore_robots + 1, ..*self }
    }

    fn _add_clay_robot(&self) -> Self {
        Self { clay_robots: self.clay_robots + 1, ..*self }
    }

    fn _add_obsidian_robot(&self) -> Self {
        Self { obsidian_robots: self.obsidian_robots + 1, ..*self }
    }

    fn _add_geode_robot(&self) -> Self {
        Self { geode_robots: self.geode_robots + 1, ..*self }
    }

    fn add_robot(&self, robot_type: &RobotType, blueprint: &Blueprint) -> Self {
        match robot_type {
            RobotType::Ore => {
                self._sub_cost(blueprint.ore_robot_cost)._add_ore_robot()
            }
            RobotType::Clay => {
                self._sub_cost(blueprint.clay_robot_cost)._add_clay_robot()
            }
            RobotType::Obsidian => {
                self._sub_cost(blueprint.obsidian_robot_cost)._add_obsidian_robot()
            }
            RobotType::Geode => {
                self._sub_cost(blueprint.geode_robot_cost)._add_geode_robot()
            }
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
        self._time_until_can_build(
        match robot_type {
            RobotType::Ore => blueprint.ore_robot_cost,
            RobotType::Clay => blueprint.clay_robot_cost,
            RobotType::Obsidian => blueprint.obsidian_robot_cost,
            RobotType::Geode => blueprint.geode_robot_cost,
        })

    }

    fn next(&self, robot_type: &RobotType, blueprint: &Blueprint) -> State {
        assert!(self.minute < 24);
        let s = match self.time_until_can_build(robot_type, blueprint) {
            Some(time) if self.minute + time <= 24 => self.after(time).add_robot(robot_type, blueprint),
            Some(_) => self.after(24 - self.minute),
            None => self.after(24 - self.minute),
        };
        // println!("   {:?}", self);
        // println!("=> {:?}", s);
        s
    }

    fn best(&self, blueprint: &Blueprint) -> i32 {
        if self.minute == 24 {
            return self.geode;
        }
        let mut b = 0;
        for robot_type in vec![RobotType::Ore, RobotType::Clay, RobotType::Obsidian, RobotType::Geode].iter().rev() {
            b = core::cmp::max(b, self.next(robot_type, blueprint).best(blueprint));
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
    fn best(&self) -> i32 {
        State::new().best(self)

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
    fn simulate(
        &self,
        last_ore_robot: usize,
        last_clay_robot: usize,
        last_obsidian_robot: usize,
    ) -> i32 {
        let mut state = State::new();
        for minute in 0..24 {
            let robot_type = if minute <= last_ore_robot {
                RobotType::Ore
            } else if minute <= last_clay_robot {
                RobotType::Clay
            } else if minute <= last_obsidian_robot {
                RobotType::Obsidian
            } else {
                RobotType::Geode
            };
            state.produce_and_try_build(robot_type, self);
        }
        state.geode
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(Blueprint::read)
        .enumerate()
        .map(|(i, b)| b.best() * (i as i32 + 1))
        .inspect(|s| println!("{:?}", s))
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day19e.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 15120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1234);
    }
}
