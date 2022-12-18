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
    let (mut minx, mut miny, mut minz) = (i32::MAX, i32::MAX, i32::MAX);
    let (mut maxx, mut maxy, mut maxz) = (i32::MIN, i32::MIN, i32::MIN);
    let mut grid = BTreeSet::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            (let x: i32, ",", let y: i32, ",", let z: i32) => {
                grid.insert((x,y,z));
                minx = x.min(minx);
                miny = y.min(miny);
                minz = z.min(minz);
                maxx = x.max(maxx);
                maxy = y.max(maxy);
                maxz = z.max(maxz);
            }
        )
        .unwrap();
    }

    let mut stack = vec![(minx - 1, miny - 1, minz - 1)];
    let mut seen = BTreeSet::new();

    while let Some((cx, cy, cz)) = stack.pop() {
        for (x, y, z) in [
            (cx + 1, cy, cz),
            (cx - 1, cy, cz),
            (cx, cy + 1, cz),
            (cx, cy - 1, cz),
            (cx, cy, cz - 1),
            (cx, cy, cz + 1),
        ] {
            if x >= minx - 1 && x <= maxx + 1
                && y >= miny - 1 && y <= maxy + 1
                && z >= minz - 1 && z <= maxz + 1
                && !seen.contains(&(x, y, z)) & !grid.contains(&(x, y, z)) {
                stack.push((x, y, z));
                seen.insert((x, y, z));
            }
        }
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
            if seen.contains(&(x, y, z)) {
                score += 1;
            }
        }
    }
    println!("{score}");
    Ok(())
}
