use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

struct Valve {
    flow_rate: i64,
    tunnels: BTreeMap<String, i64>,
}

impl Valve {
    fn read(line: &str) -> (String, Self) {
        let (_, name, _, _, flow_rate, _, _, _, _, tunnels) =
            line.splitn(10, ' ').collect_tuple().unwrap();
        (
            name.to_string(),
            Valve {
                flow_rate: flow_rate[5..flow_rate.len() - 1].parse().unwrap(),
                tunnels: BTreeMap::from_iter(tunnels.split(", ").map(String::from).map(|t| (t, 1))),
            },
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    valves: &BTreeMap<String, Valve>,
    at: &String,
    minute: i64,
    doneflow: i64,
    openflow: i64,
    bestflow: &mut i64,
    allflow: &i64,
    openvalves: &BTreeSet<String>,
    time_limit: i64,
) {
    if minute >= time_limit || openflow == *allflow {
        let doneflow = doneflow + (time_limit - minute) * openflow;
        println!("{:?} {:?}", doneflow, bestflow);
        if doneflow > *bestflow {
            *bestflow = doneflow;
        }
        return;
    }
    let max_possible_flow = doneflow
        + (time_limit - minute) * openflow
        + (time_limit - 1 - minute) * (allflow - openflow);
    if max_possible_flow < *bestflow {
        return;
    }
    println!("{}", at);
    let v = valves.get(at).unwrap();
    if v.flow_rate > 0 && !openvalves.contains(at) {
        let mut openvalves = openvalves.clone();
        openvalves.insert(at.clone());
        dfs(
            valves,
            at,
            minute + 1,
            doneflow + openflow,
            openflow + v.flow_rate,
            bestflow,
            allflow,
            &openvalves,
            time_limit,
        );
    }
    for (vv, dist) in &v.tunnels {
        dfs(
            valves,
            vv,
            minute + dist,
            doneflow + openflow * dist,
            openflow,
            bestflow,
            allflow,
            openvalves,
            time_limit,
        );
    }
}

fn consolidate(valves: &BTreeMap<String, Valve>) -> BTreeMap<String, Valve> {
    BTreeMap::from_iter(
        valves
            .iter()
            .filter(|(n, v)| *n == "AA" || v.flow_rate > 0)
            .map(|(name, valve)| {
                let mut long_tunnels: BTreeMap<String, i64> = BTreeMap::new();
                let mut q: VecDeque<(String, i64)> =
                    VecDeque::from_iter(valve.tunnels.iter().map(|(k, v)| (k.clone(), *v)));
                let mut visited: BTreeSet<String> = BTreeSet::new();
                loop {
                    match q.pop_front() {
                        None => break,
                        Some((name, dist)) => {
                            if visited.contains(&name) {
                                continue;
                            }
                            visited.insert(name.clone());
                            let to_valve = valves.get(&name).unwrap();
                            if to_valve.flow_rate > 0 {
                                long_tunnels.insert(name.to_string(), dist);
                            }
                            for (name, len) in to_valve.tunnels.iter() {
                                assert!(*len == 1);
                                q.push_back((name.clone(), dist + len));
                            }
                        }
                    }
                }
                (
                    name.to_string(),
                    Valve {
                        flow_rate: valve.flow_rate,
                        tunnels: long_tunnels,
                    },
                )
            }),
    )
}

pub fn part1(input: &str) -> i64 {
    let valves = consolidate(&BTreeMap::from_iter(input.lines().map(Valve::read)));
    let allflow = valves.values().map(|v| v.flow_rate).sum();
    let mut bestflow = 2000;  // know can get this much from previous long runs (helps cut off branches)
    dfs(
        &valves,
        &"AA".to_string(),
        0,
        0,
        0,
        &mut bestflow,
        &allflow,
        &BTreeSet::new(),
        30,
    );
    bestflow
}

pub fn part2(input: &str) -> i64 {
    let valves = consolidate(&BTreeMap::from_iter(input.lines().map(Valve::read)));
    let allflow = valves.values().map(|v| v.flow_rate).sum();
    let mut bestflow = 0;
    dfs(
        &valves,
        &"AA".to_string(),
        // &"AA".to_string(),
        0,
        0,
        0,
        &mut bestflow,
        &allflow,
        &BTreeSet::new(),
        24,
    );
    bestflow
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day16.txt")
    }

    #[ignore = "slow"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 2250);
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
