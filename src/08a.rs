// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut area = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let chars: Vec<i32> = line.trim().chars().map(|c| c as i32 - '0' as i32).collect();
        area.push(chars);
    }
    let mut visible = HashSet::new();

    let rows = area.len();
    let cols = area[0].len();

    for row in 0..rows {
        let mut max = -1;
        for col in 0..cols {
            let height = area[row][col];
            if height > max {
                max = height;
                visible.insert((row, col));
            }
        }
    }

    for row in 0..rows {
        let mut max = -1;
        for col in (0..cols).rev() {
            let height = area[row][col];
            if height > max {
                max = height;
                visible.insert((row, col));
            }
        }
    }

    for col in 0..cols {
        let mut max = -1;
        for row in 0..rows {
            let height = area[row][col];
            if height > max {
                max = height;
                visible.insert((row, col));
            }
        }
    }

    for col in 0..cols {
        let mut max = -1;
        for row in (0..rows).rev() {
            let height = area[row][col];
            if height > max {
                max = height;
                visible.insert((row, col));
            }
        }
    }
    println!("{}", visible.len());

    Ok(())
}
