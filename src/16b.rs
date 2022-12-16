// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use fixedbitset::FixedBitSet;
use gray_codes::Subsets;
use itertools::Itertools;
use rayon::prelude::*;
use scan_rules::scan;
use scan_rules::scanner::Word;
use std::collections::BTreeMap;
use std::fmt;
use std::io::BufRead;
use std::str;

#[macro_use]
extern crate scan_rules;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveId(u16);

impl From<&str> for ValveId {
    fn from(id: &str) -> Self {
        let a = *id.as_bytes().first().unwrap() as u16 - 'A' as u16;
        let b = *id.as_bytes().get(1).unwrap() as u16- 'A' as u16;
        Self(a *26 + b)
    }
}

impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.0 /26;
        let b = self.0%26;
        f.write_str(str::from_utf8(&[a as u8, b as u8]).unwrap())
    }
}

struct ValveIdSet(FixedBitSet);

impl ValveIdSet {
    pub fn new() -> Self {
        Self(FixedBitSet::with_capacity(26*26))
    }

    pub fn from_vec(vec: Vec<&ValveId>) -> Self {
        let mut set = Self::new();
        for v in vec {
            set.insert(*v);
        }
        set
    }

    #[inline(always)]
    pub fn insert(&mut self, value: ValveId) {
        self.0.insert(value.0 as usize)
    }

    #[inline(always)]
    pub fn remove(&mut self, value: &ValveId) {
        self.0.set(value.0 as usize, false);
    }

    #[inline(always)]
    pub fn contains(&mut self, value: &ValveId) -> bool{
        self.0.contains(value.0 as usize)
    }
    
    // pub fn iter(&self) -> impl Iterator + '_{
    //     self.0.ones()
    // }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Valve {
    id: ValveId,
    flow_rate: i64,
    edges: Vec<ValveId>,
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

    let count_release = |time_left: i64, cur: ValveId, candidate: ValveId| {
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
        cur: ValveId,
        time_left: i64,
        visited: &mut ValveIdSet,
        cur_score: i64,
        valves_to_consider: &Vec<ValveId>,
        count_release: &T,
    ) -> i64
    where
        T: Fn(i64, ValveId, ValveId) -> (i64, i64) + Send + Sync,
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

    let result = Subsets::<ValveId, Vec<&ValveId>>::of(&non_zero)
        .filter(|s| s.len() <= 13)
        .par_bridge()
        .map(|s| {
            let me = visit(
                valves[&"AA".into()].id,
                26,
                &mut ValveIdSet::new(),
                0,
                &s.iter().map(|e| **e).collect(),
                &count_release,
            );
            let valves_to_consider: Vec<_> = non_zero.iter().filter(|v| !s.contains(v)).cloned().collect();
            visit(
                valves[&"AA".into()].id,
                26,
                &mut ValveIdSet::from_vec(s),
                me,
                &valves_to_consider,
                &count_release,
            )
        })
        .max()
        .unwrap();

    println!("{result}");

    Ok(())
}
