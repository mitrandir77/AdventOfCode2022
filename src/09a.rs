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
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited.insert(head_pos);
    while let Ok((dir, steps)) = read_move() {
        for _step in 0..steps {
            let mv = match dir {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("wrong direction"),
            };
            head_pos.0 += mv.0;
            head_pos.1 += mv.1;

            if ((tail_pos.0 - head_pos.0) as i32).abs() >= 2
                || ((tail_pos.1 - head_pos.1) as i32).abs() >= 2
            {
                if tail_pos.0 > head_pos.0 {
                    tail_pos.0 -= 1;
                }
                if tail_pos.0 < head_pos.0 {
                    tail_pos.0 += 1;
                }
                if tail_pos.1 > head_pos.1 {
                    tail_pos.1 -= 1;
                }
                if tail_pos.1 < head_pos.1 {
                    tail_pos.1 += 1;
                }
                let _updated = visited.insert(tail_pos);
            }
        }
    }

    println!("{}", visited.len());

    Ok(())
}
