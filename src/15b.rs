// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
use std::collections::BTreeSet;

use anyhow::Result;
use text_io::try_scan;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn read_positions() -> Result<(Point, Point)> {
    let sx: i64;
    let sy: i64;
    let bx: i64;
    let by: i64;
    try_scan!(
        "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
        sx,
        sy,
        bx,
        by
    );
    Ok((Point { x: sx, y: sy }, Point { x: bx, y: by }))
}

fn main() -> Result<()> {
    let mut input = Vec::new();
    while let Ok((s, b)) = read_positions() {
        input.push((s, b));
    }

    for ycheck in 0..=4000000 {
        let mut intervals = Vec::new();
        let mut beacons_at_y = BTreeSet::new();
        for (s, b) in input.iter() {
            if b.y == ycheck {
                beacons_at_y.insert(b.x);
            }
            let dist = (s.x - b.x).abs() + (s.y - b.y).abs();
            let distx_max = dist - (s.y - ycheck).abs();

            if distx_max > 0 {
                intervals.push((s.x - distx_max, s.x + distx_max));
            }
        }

        intervals.sort();

        let (mut la, mut lb) = intervals[0];

        let mut le = lb;
        for (a, b) in &intervals[1..] {
            if *a > lb {
                if la > le + 1 && !beacons_at_y.contains(&(le + 1)) && le + 1 >= 0 && le < 4000000 {
                    println!("{}", 4000000*(le+1)+ycheck);
                }
                le = lb;
                la = *a;
                lb = *b;
            } else {
                lb = lb.max(*b);
            }
        }
        if la > le + 1 && !beacons_at_y.contains(&(le + 1)) && le + 1 >= 0 && le < 4000000 {
                    println!("{}", 4000000*(le+1)+ycheck);
        }
    }

    Ok(())
}
