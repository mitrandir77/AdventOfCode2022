// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use rayon::prelude::*;
use scan_rules::scan;
use std::collections::{BTreeSet, HashMap};
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    ore_robot_cost_in_ore: i32,
    clay_robot_cost_in_ore: i32,
    obsidian_robot_cost_in_ore: i32,
    obsidian_robot_cost_in_clay: i32,
    geode_robot_cost_in_obsidian: i32,
    geode_robot_cost_in_ore: i32,
}

#[derive(Debug, Clone)]
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
    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }
}

fn find_max_geodes_num(mut state: State, blueprint: &Blueprint, time_left: i32) -> i32 {
    if time_left == 0 {
        return state.geode;
    }

    let mut max = 0;
    if state.ore >= blueprint.geode_robot_cost_in_ore
        && state.obsidian >= blueprint.geode_robot_cost_in_obsidian
    {
        let mut state = state.clone();
        state.collect();
        state.ore -= blueprint.geode_robot_cost_in_ore;
        state.obsidian -= blueprint.geode_robot_cost_in_obsidian;
        state.geode_robots += 1;
        max = max.max(find_max_geodes_num(state, blueprint, time_left - 1));
    }
    else if state.ore >= blueprint.obsidian_robot_cost_in_ore
        && state.clay >= blueprint.obsidian_robot_cost_in_clay
    {
        let mut state = state.clone();
        state.collect();
        state.ore -= blueprint.obsidian_robot_cost_in_ore;
        state.clay -= blueprint.obsidian_robot_cost_in_clay;
        state.obsidian_robots += 1;
        max = max.max(find_max_geodes_num(state, blueprint, time_left - 1));
    }
    else if state.ore >= blueprint.clay_robot_cost_in_ore {
        let mut state = state.clone();
        state.collect();
        state.ore -= blueprint.clay_robot_cost_in_ore;
        state.clay_robots += 1;
        max = max.max(find_max_geodes_num(state, blueprint, time_left - 1));
    }

    if state.ore >= blueprint.ore_robot_cost_in_ore && time_left > blueprint.ore_robot_cost_in_ore {
        let mut state = state.clone();
        state.collect();
        state.ore -= blueprint.ore_robot_cost_in_ore;
        state.ore_robots += 1;
        max = max.max(find_max_geodes_num(state, blueprint, time_left - 1));
    }
    state.collect();
    max = max.max(find_max_geodes_num(state, blueprint, time_left - 1));

    max
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut blueprints = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            ("Blueprint ", let id: i32, ": Each ore robot costs ", let ore_robot_cost_in_ore: i32, " ore. Each clay robot costs ", let clay_robot_cost_in_ore: i32, " ore. Each obsidian robot costs ", let obsidian_robot_cost_in_ore :i32, " ore and ", let obsidian_robot_cost_in_clay: i32," clay. Each geode robot costs ",let geode_robot_cost_in_ore: i32," ore and ", let geode_robot_cost_in_obsidian: i32," obsidian.") => {
                blueprints.push(Blueprint {
                    id,
                    ore_robot_cost_in_ore,
                    clay_robot_cost_in_ore,
                    obsidian_robot_cost_in_clay,
                    obsidian_robot_cost_in_ore,
                    geode_robot_cost_in_obsidian,
                    geode_robot_cost_in_ore,
                });
            }
        )
        .unwrap();
    }

    let score: i32 = blueprints
        .into_iter()
        .par_bridge()
        .map(|b| {
            b.id * find_max_geodes_num(
                State {
                    ore_robots: 1,
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                &b,
                24,
            )
        })
        .sum();
    println!("{score}");

    Ok(())
}
