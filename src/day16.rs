use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;

struct Valve_ {
    flow_rate: i64,
    tunnels: BTreeMap<String, i64>,
}

impl Valve_ {
    fn read(line: &str) -> (String, Self) {
        let (_, name, _, _, flow_rate, _, _, _, _, tunnels) =
            line.splitn(10, ' ').collect_tuple().unwrap();
        (
            name.to_string(),
            Self {
                flow_rate: flow_rate[5..flow_rate.len() - 1].parse().unwrap(),
                tunnels: BTreeMap::from_iter(tunnels.split(", ").map(String::from).map(|t| (t, 1))),
            },
        )
    }
    fn to_indexified(&self, name_to_index: &BTreeMap<String, usize>) -> Valve {
        Valve {
            flow_rate: self.flow_rate,
            tunnels: BTreeMap::from_iter(
                self.tunnels
                    .iter()
                    .map(|(name, dist)| (*name_to_index.get(name).unwrap(), *dist)),
            ),
        }
    }
}

struct Valve {
    flow_rate: i64,
    tunnels: BTreeMap<usize, i64>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct ValvesState {
    open_valves_bitmask: u64,
}

impl std::ops::BitOr<ValvesState> for ValvesState {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self {
            open_valves_bitmask: self.open_valves_bitmask | other.open_valves_bitmask,
        }
    }
}
impl std::ops::BitAnd<ValvesState> for ValvesState {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self {
            open_valves_bitmask: self.open_valves_bitmask & other.open_valves_bitmask,
        }
    }
}
impl ValvesState {
    fn new_with_open_valve(i: usize) -> Self {
        Self {
            open_valves_bitmask: 1 << i,
        }
    }
    fn with_open_valve(&self, i: usize) -> Self {
        *self | Self::new_with_open_valve(i)
    }
    fn is_valve_open(&self, i: usize) -> bool {
        (*self & Self::new_with_open_valve(i)).any_valves_open()
    }
    fn any_valves_open(&self) -> bool {
        self.open_valves_bitmask != 0
    }
    fn new_all_valves_closed() -> Self {
        Self {
            open_valves_bitmask: 0,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    valves: &Vec<Valve>,
    at: usize,
    minute: i64,
    doneflow: i64,
    openflow: i64,
    bestflow: &mut i64,
    allflow: &i64,
    openvalves: ValvesState,
    time_limit: i64,
) {
    if minute >= time_limit || openflow == *allflow {
        let doneflow = doneflow + (time_limit - minute) * openflow;
        //println!("{:?} {:?}", doneflow, bestflow);
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
    //println!("{}", at);
    let v = valves.get(at).unwrap();
    if v.flow_rate > 0 && !openvalves.is_valve_open(at) {
        dfs(
            valves,
            at,
            minute + 1,
            doneflow + openflow,
            openflow + v.flow_rate,
            bestflow,
            allflow,
            openvalves.with_open_valve(at),
            time_limit,
        );
    }
    for (vv, dist) in &v.tunnels {
        dfs(
            valves,
            *vv,
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

fn indexify(valves: &BTreeMap<String, Valve_>) -> (usize, Vec<Valve>) {
    let names = valves.keys().enumerate().collect_vec();
    let name_to_index =
        BTreeMap::from_iter(names.iter().map(|(i, name)| (String::from(*name), *i)));
    (
        *name_to_index.get("AA").unwrap(),
        names
            .iter()
            .map(|(_, name)| valves.get(*name).unwrap().to_indexified(&name_to_index))
            .collect_vec(),
    )
}

fn consolidate(valves: &BTreeMap<String, Valve_>) -> BTreeMap<String, Valve_> {
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
                    Valve_ {
                        flow_rate: valve.flow_rate,
                        tunnels: long_tunnels,
                    },
                )
            }),
    )
}

pub fn part1(input: &str) -> i64 {
    let (aa_valve, valves) = indexify(&consolidate(&BTreeMap::from_iter(
        input.lines().map(Valve_::read),
    )));
    let allflow = valves.iter().map(|v| v.flow_rate).sum();
    let mut bestflow = 0; // know can get this much from previous long runs (helps cut off branches)
    dfs(
        &valves,
        aa_valve,
        0,
        0,
        0,
        &mut bestflow,
        &allflow,
        ValvesState::new_all_valves_closed(),
        30,
    );
    bestflow
}

pub fn part2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day16e.txt")
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
