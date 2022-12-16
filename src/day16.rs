use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

struct Valve {
    flow_rate: i64,
    tunnels: Vec<String>,
}

impl Valve {
    fn read(line: &str) -> (String, Self) {
        let (_, name, _, _, flow_rate, _, _, _, _, tunnels) = line.splitn(10, ' ').collect_tuple().unwrap();
        (name.to_string(), Valve {
            flow_rate: flow_rate[5..flow_rate.len()-1].parse().unwrap(),
            tunnels: tunnels.split(", ").map(String::from).collect_vec(),
        })
    }
}

fn dfs(valves: &BTreeMap<String, Valve>, at: &String, minute: i64, doneflow: i64, openflow: i64, bestflow: &mut i64, allflow: &i64, openvalves: &BTreeSet<String>) {
    if minute == 30 || openflow == *allflow {
        let doneflow = doneflow + (30 - minute) * openflow;
        println!("{:?} {:?}", doneflow, bestflow);
        if doneflow > *bestflow {
            *bestflow = doneflow;
        }
        return;
    }
    let max_possible_flow = doneflow + (30 - minute) * openflow + (29 - minute) * (allflow - openflow);
    if max_possible_flow < *bestflow {
        return;
    }
    let v = valves.get(at).unwrap();
    if v.flow_rate > 0 && !openvalves.contains(at) {
        let mut openvalves = openvalves.clone();
        openvalves.insert(at.clone());
        dfs(valves, at, minute + 1, doneflow + openflow, openflow + v.flow_rate, bestflow, allflow, &openvalves);
    }
    for vv in &v.tunnels {
        dfs(valves, vv, minute + 1, doneflow + openflow, openflow, bestflow, allflow, openvalves);
    }
}

pub fn part1(input: &str) -> i64 {
    let valves = BTreeMap::from_iter(input.lines().map(Valve::read));
    let allflow = valves.iter().map(|(_, v)| v.flow_rate).sum();
    let mut bestflow = 1500;
    dfs(&valves, &"AA".to_string(), 0, 0, 0, &mut bestflow, &allflow, &BTreeSet::new());
    bestflow
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day16.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
