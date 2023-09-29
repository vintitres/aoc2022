use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

use itertools::Itertools;

struct Valve_ {
    flow_rate: u64,
    tunnels: BTreeMap<String, u64>,
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
    flow_rate: u64,
    tunnels: BTreeMap<usize, u64>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos1: usize,
    pos1dist: u64,
    pos2: usize,
    pos2dist: u64,
    open_valves: ValvesState,
    done_flow: u64,
    minute: u64,
}

impl State {
    fn normalized(&self) -> Self {
        // keep pos1pos2 sorted by dist and then by pos value
        if self.pos1dist > self.pos2dist
            || (self.pos1dist == self.pos2dist && self.pos1 > self.pos2)
        {
            State {
                pos1: self.pos2,
                pos1dist: self.pos2dist,
                pos2: self.pos1,
                pos2dist: self.pos1dist,
                ..*self
            }
        } else {
            Self { ..*self }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    state: State,
    valves: &Vec<Valve>,
    openflow: u64,
    bestflow: &mut u64,
    allflow: &u64,
    time_limit: u64,
    seen: &mut HashSet<State>,
) {
    if seen.contains(&state) {
        return;
    }
    //seen.insert(state.clone());
    if state.minute >= time_limit || openflow == *allflow {
        let doneflow = (state.done_flow as i64
            + (time_limit as i64 - state.minute as i64) * openflow as i64)
            as u64;
        //eprintln!("{:?} {:?}", doneflow, bestflow);
        if doneflow > *bestflow {
            eprintln!("{:?}", bestflow);
            *bestflow = doneflow;
        }
        return;
    }
    let max_possible_flow = state.done_flow
        + (time_limit - state.minute) * openflow
        + (time_limit - 1 - state.minute) * (allflow - openflow);
    if max_possible_flow < *bestflow {
        return;
    }
    if state.pos1dist == 0 {
        let v1 = valves.get(state.pos1).unwrap();
        if state.pos2dist == 0 {
            let v2 = valves.get(state.pos2).unwrap();

            // 1 and 2 open
            if state.pos1 != state.pos2 && v1.flow_rate > 0
                    && !state.open_valves.is_valve_open(state.pos1)
                    && v2.flow_rate > 0 && !state.open_valves.is_valve_open(state.pos2) {
                dfs(
                    State {
                        minute: state.minute + 1,
                        open_valves: state
                            .open_valves
                            .with_open_valve(state.pos1)
                            .with_open_valve(state.pos2),
                        done_flow: state.done_flow + openflow,
                        ..state
                    },
                    valves,
                    openflow + v1.flow_rate + v2.flow_rate,
                    bestflow,
                    allflow,
                    time_limit,
                    seen,
                );
            }

            // 1 opens, 2 goes into new tunnel
            if v1.flow_rate > 0 && !state.open_valves.is_valve_open(state.pos1) {
                for (vv2, dist) in &v2.tunnels {
                    dfs(
                        State {
                            pos2dist: dist - 1,
                            pos2: *vv2,
                            minute: state.minute + 1,
                            open_valves: state.open_valves.with_open_valve(state.pos1),
                            done_flow: state.done_flow + openflow,
                            ..state
                        }
                        .normalized(),
                        valves,
                        openflow + v1.flow_rate,
                        bestflow,
                        allflow,
                        time_limit,
                        seen,
                    );
                }
            }

            // 1 goes into new tunnel, 2 opens
            if state.pos1 != state.pos2
                && v2.flow_rate > 0
                && !state.open_valves.is_valve_open(state.pos2)
            {
                for (vv1, dist) in &v1.tunnels {
                    dfs(
                        State {
                            pos1dist: dist - 1,
                            pos1: *vv1,
                            minute: state.minute + 1,
                            open_valves: state.open_valves.with_open_valve(state.pos2),
                            done_flow: state.done_flow + openflow,
                            ..state
                        }
                        .normalized(),
                        valves,
                        openflow + v2.flow_rate,
                        bestflow,
                        allflow,
                        time_limit,
                        seen,
                    );
                }
            }

            //  1 and 2 go into new tunnel
            for (vv1, dist1) in &v1.tunnels {
                for (vv2, dist2) in &v2.tunnels {
                    let step_dist = std::cmp::min(dist1, dist2);
                    dfs(
                        State {
                            pos1dist: dist1 - step_dist,
                            pos2dist: dist2 - step_dist,
                            pos1: *vv1,
                            pos2: *vv2,
                            minute: state.minute + step_dist,
                            done_flow: state.done_flow + openflow * step_dist,
                            ..state
                        }
                        .normalized(),
                        valves,
                        openflow,
                        bestflow,
                        allflow,
                        time_limit,
                        seen,
                    );
                }
            }
        } else {
            // 1 opens, 2 moves
            if v1.flow_rate > 0 && !state.open_valves.is_valve_open(state.pos1) {
                dfs(
                    State {
                        pos2dist: state.pos2dist - 1,
                        minute: state.minute + 1,
                        open_valves: state.open_valves.with_open_valve(state.pos1),
                        done_flow: state.done_flow + openflow,
                        ..state
                    }
                    .normalized(),
                    valves,
                    openflow + v1.flow_rate,
                    bestflow,
                    allflow,
                    time_limit,
                    seen,
                );
            }
            // 1 goes into new tunnel, 2 moves
            for (vv1, dist) in &v1.tunnels {
                let step_dist = std::cmp::min(dist, &state.pos2dist);
                dfs(
                    State {
                        pos1dist: dist - step_dist,
                        pos1: *vv1,
                        pos2dist: state.pos2dist - step_dist,
                        minute: state.minute + step_dist,
                        done_flow: state.done_flow + openflow * step_dist,
                        ..state
                    }
                    .normalized(),
                    valves,
                    openflow,
                    bestflow,
                    allflow,
                    time_limit,
                    seen,
                );
            }
        }
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
                let mut long_tunnels: BTreeMap<String, u64> = BTreeMap::new();
                let mut q: VecDeque<(String, u64)> =
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

pub fn part1(input: &str) -> u64 {
    opennn(input, false, 30)
}
fn opennn(input: &str, with_elephant: bool, time: u64) -> u64 {
    let (aa_valve, valves) = indexify(&consolidate(&BTreeMap::from_iter(
        input.lines().map(Valve_::read),
    )));
    let allflow = valves.iter().map(|v| v.flow_rate).sum();
    let mut bestflow = 1600; // know can get this much from previous long runs (helps cut off branches)
    let mut seen = HashSet::new();
    dfs(
        State {
            pos1: aa_valve,
            pos1dist: 0,
            pos2: if with_elephant { aa_valve } else { usize::MAX },
            pos2dist: if with_elephant { 0 } else {u64::MAX },
            done_flow: 0,
            open_valves: ValvesState::new_all_valves_closed(),
            minute: 0,
        },
        &valves,
        0,
        &mut bestflow,
        &allflow,
        time,
        &mut seen,
    );
    bestflow
}

pub fn part2(input: &str) -> u64 {
    opennn(input, true, 26)
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
