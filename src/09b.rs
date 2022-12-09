// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeSet;
use text_io::try_scan;

fn read_move() -> Result<(char, i32)> {
    let dir: char;
    let steps: i32;
    try_scan!("{} {}", dir, steps);
    Ok((dir, steps))
}

fn main() -> Result<()> {
    let mut visited = BTreeSet::new();
    let mut knots = vec![(0, 0); 10];
    visited.insert(knots[9]);
    while let Ok((dir, steps)) = read_move() {
        for _step in 0..steps {
            let mv = match dir {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("wrong direction"),
            };
            knots[0].0 += mv.0;
            knots[0].1 += mv.1;

            for (h, t) in (0..9).zip(1..) {
                if ((knots[t].0 - knots[h].0) as i32).abs() >= 2
                    || ((knots[t].1 - knots[h].1) as i32).abs() >= 2
                {
                    if knots[t].0 > knots[h].0 {
                        knots[t].0 -= 1;
                    }
                    if knots[t].0 < knots[h].0 {
                        knots[t].0 += 1;
                    }
                    if knots[t].1 > knots[h].1 {
                        knots[t].1 -= 1;
                    }
                    if knots[t].1 < knots[h].1 {
                        knots[t].1 += 1;
                    }
                }
            }
            let _updated = visited.insert(knots[9]);
        }
    }

    println!("{}", visited.len());

    Ok(())
}
