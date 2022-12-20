use itertools::Itertools;

type RobotCost = (i32, i32, i32);

enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
struct State {
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
