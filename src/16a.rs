// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use scan_rules::scan;
use scan_rules::scanner::Word;
use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;
use std::str;

#[macro_use]
extern crate scan_rules;
use smallset::SmallSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id(u8, u8);

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        Self(
            *id.as_bytes().first().unwrap(),
            *id.as_bytes().get(1).unwrap(),
        )
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(str::from_utf8(&[self.0, self.1]).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Valve {
    id: Id,
    flow_rate: i64,
    edges: Vec<Id>,
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut valves = BTreeMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            ("Valve ", let id: Word<String>, " has flow rate=", let flow_rate: i64, "; tunnels lead to valves ", [ let edges: Word<String>],+: Vec<_>) => {
                let valve = Valve {
                    id: id.as_str().into(),
                    flow_rate,
                    edges: edges.iter().map(|s| s.as_str().into()).collect_vec()
                };
                valves.insert(valve.id, valve);
            },
            ("Valve ", let id: Word<String>, " has flow rate=", let flow_rate: i64, "; tunnel leads to valve ", [ let edges: Word<String>],+: Vec<_>) => {
                let valve = Valve {
                    id: id.as_str().into(),
                    flow_rate,
                    edges: edges.iter().map(|s|s.as_str().into()).collect_vec()
                };
                valves.insert(valve.id, valve);
            },
        )
        .unwrap();
    }

    let mut keys: Vec<_> = valves.keys().cloned().collect();
    keys.sort();

    let mut dist = BTreeMap::new();
    for i in &keys {
        for j in &keys {
            dist.insert((*i, *j), 10000);
        }
        dist.insert((*i, *i), 0);
    }

    for valve in valves.values() {
        for edge in valve.edges.iter() {
            dist.insert((valve.id, *edge), 1);
        }
    }

    for k in &keys {
        for i in &keys {
            for j in &keys {
                if dist[&(*i, *j)] > dist[&(*i, *k)] + dist[&(*k, *j)] {
                    dist.insert((*i, *j), dist[&(*i, *k)] + dist[&(*k, *j)]);
                }
            }
        }
    }

    let count_release = |time_left: i64, cur: Id, candidate: Id| {
        let time_consumed = dist[&(cur, candidate)] + 1;
        let time_left = time_left - time_consumed;
        (valves[&candidate].flow_rate * time_left, time_left)
    };

    let non_zero: Vec<_> = valves
        .values()
        .filter(|v| (v.flow_rate > 0))
        .map(|v| v.id)
        .collect();

    fn visit<T>(
        cur: Id,
        time_left: i64,
        visited: &mut SmallSet<[Id; 14]>,
        cur_score: i64,
        valves_to_consider: &Vec<Id>,
        count_release: &T,
    ) -> i64
    where
        T: Fn(i64, Id, Id) -> (i64, i64) + Send + Sync,
    {
        visited.insert(cur);

        let mut res = cur_score;
        for v in valves_to_consider.iter() {
            if !visited.contains(v) {
                let (release_value, new_time_left) = count_release(time_left, cur, *v);
                if new_time_left > 0 {
                    res = res.max(visit(
                        *v,
                        new_time_left,
                        visited,
                        cur_score + release_value,
                        valves_to_consider,
                        count_release,
                    ))
                }
            }
        }
        visited.remove(&cur);
        res
    }

    let result = visit(
        valves[&"AA".into()].id,
        30,
        &mut SmallSet::new(),
        0,
        &non_zero,
        &count_release,
    );

    println!("{result}");

    Ok(())
}
