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
    let ycheck = 2000000;
    let mut intervals = Vec::new();
    let mut beacons_at_y = BTreeSet::new();
    while let Ok((s, b)) = read_positions() {
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

    let mut res = 0;
    let (mut la, mut lb) = intervals[0];

    for (a, b) in &intervals[1..] {
        if *a > lb {
            while let Some(beacon) = beacons_at_y.first() {
                if beacon > &lb {
                    break;
                }
                let beacon = beacons_at_y.pop_first().unwrap();

                if beacon >= la {
                    res -= 1;
                }
            }
            res += lb - la + 1;
            la = *a;
            lb = *b;
        } else {
            lb = lb.max(*b);
        }
    }
    while let Some(beacon) = beacons_at_y.first() {
        if beacon > &lb {
            break;
        }
        let beacon = beacons_at_y.pop_first().unwrap();

        if beacon >= la {
            res -= 1;
        }
    }
    res += lb - la + 1;

    println!("{res}");

    Ok(())
}
