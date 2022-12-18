// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scan;
use std::collections::BTreeSet;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;
fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut score: usize = 0;
    let mut grid = BTreeSet::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            (let x: i32, ",", let y: i32, ",", let z: i32) => {
                grid.insert((x,y,z));
            }
        )
        .unwrap();
    }

    for (cx, cy, cz) in &grid {
        for (x, y, z) in [
            (cx + 1, *cy, *cz),
            (cx - 1, *cy, *cz),
            (*cx, cy + 1, *cz),
            (*cx, cy - 1, *cz),
            (*cx, *cy, cz - 1),
            (*cx, *cy, cz + 1),
        ] {
            if grid.contains(&(x, y, z)) {
                continue;
            }
            score += 1;
        }
    }
    println!("{score}");
    Ok(())
}
