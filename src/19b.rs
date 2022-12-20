// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use rayon::prelude::*;
use scan_rules::scan;
use std::io::BufRead;
use strum::EnumIter;
use strum::IntoEnumIterator;
#[macro_use]
extern crate scan_rules;

#[derive(Debug, Clone)]
struct Blueprint {
    _id: i32,
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
    ore_robots_limit: i32,
    clay_robots_limit: i32,
    obsidian_robots_limit: i32,
}

impl State {
    fn collect(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }
}

#[derive(Clone, Copy, EnumIter, Debug, PartialEq, Eq, Hash)]
enum Build {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn find_max_geodes_num(
    mut state: State,
    blueprint: &Blueprint,
    time_left: i32,
    next_build: Build,
) -> i32 {
    if time_left == 0 {
        // dbg!(state.geode);
        return state.geode;
    }
    // dbg!(next_build, time_left);
    match next_build {
        Build::Ore if state.ore >= blueprint.ore_robot_cost_in_ore => {
            state.collect();
            state.ore -= blueprint.ore_robot_cost_in_ore;
            state.ore_robots += 1;
        }
        Build::Clay if state.ore >= blueprint.clay_robot_cost_in_ore => {
            state.collect();
            state.ore -= blueprint.clay_robot_cost_in_ore;
            state.clay_robots += 1;
        }
        Build::Obsidian
            if state.ore >= blueprint.obsidian_robot_cost_in_ore
                && state.clay >= blueprint.obsidian_robot_cost_in_clay =>
        {
            state.collect();
            state.ore -= blueprint.obsidian_robot_cost_in_ore;
            state.clay -= blueprint.obsidian_robot_cost_in_clay;
            state.obsidian_robots += 1;
        }
        Build::Geode
            if state.ore >= blueprint.geode_robot_cost_in_ore
                && state.obsidian >= blueprint.geode_robot_cost_in_obsidian =>
        {
            state.collect();
            state.ore -= blueprint.geode_robot_cost_in_ore;
            state.obsidian -= blueprint.geode_robot_cost_in_obsidian;
            state.geode_robots += 1;
        }
        _ => {
            state.collect();
            return find_max_geodes_num(state, blueprint, time_left - 1, next_build);
        }
    }
    Build::iter()
        .filter_map(|next_build: Build| match next_build {
            Build::Ore
                if state.ore_robots < state.ore_robots_limit =>
            {
                Some(find_max_geodes_num(
                    state.clone(),
                    blueprint,
                    time_left - 1,
                    next_build,
                ))
            }
            Build::Clay
                if state.ore_robots > 0
                    && state.clay_robots < state.clay_robots_limit =>
            {
                Some(find_max_geodes_num(
                    state.clone(),
                    blueprint,
                    time_left - 1,
                    next_build,
                ))
            }
            Build::Obsidian
                if state.clay_robots > 0 && state.obsidian_robots < state.obsidian_robots_limit =>
            {
                Some(find_max_geodes_num(
                    state.clone(),
                    blueprint,
                    time_left - 1,
                    next_build,
                ))
            }
            Build::Geode if state.obsidian_robots > 0 && state.ore_robots > 0 => Some(
                find_max_geodes_num(state.clone(), blueprint, time_left - 1, next_build),
            ),
            _ => None,
        })
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut blueprints = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            ("Blueprint ", let id: i32, ": Each ore robot costs ", let ore_robot_cost_in_ore: i32, " ore. Each clay robot costs ", let clay_robot_cost_in_ore: i32, " ore. Each obsidian robot costs ", let obsidian_robot_cost_in_ore :i32, " ore and ", let obsidian_robot_cost_in_clay: i32," clay. Each geode robot costs ",let geode_robot_cost_in_ore: i32," ore and ", let geode_robot_cost_in_obsidian: i32," obsidian.") => {
                blueprints.push(Blueprint {
                    _id: id,
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
        .take(3)
        .par_bridge()
        .map(|b| {
            let state = State {
                ore_robots: 1,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
                ore_robots_limit: b.ore_robot_cost_in_ore.max(b.clay_robot_cost_in_ore).max(b.obsidian_robot_cost_in_ore),
                clay_robots_limit: b.obsidian_robot_cost_in_clay,
                obsidian_robots_limit: b.geode_robot_cost_in_obsidian,
            };
            let max = [Build::Ore, Build::Clay]
                .into_iter()
                .par_bridge()
                .map(|next_build: Build| find_max_geodes_num(state.clone(), &b, 32, next_build))
                .max()
                .unwrap();
            dbg!(max);
            max
        })
        .product();
    println!("{score}");

    Ok(())
}
