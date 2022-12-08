// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use itertools::max;
use itertools::min;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut area = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let chars: Vec<i32> = line.trim().chars().map(|c| c as i32 - '0' as i32).collect();
        area.push(chars);
    }
    let mut score = HashMap::new();

    let rows = area.len();
    let cols = area[0].len();

    for row in 0..rows {
        let mut heights = BTreeMap::new();
        for col in 0..cols {
            let height = area[row][col];

            let next_larger = max(heights.range(height..).map(|(_k, v)| *v)).unwrap_or(0);
            score.insert((row, col), col as i32 - next_larger as i32);
            heights.insert(height, col);
        }
    }

    for col in 0..cols {
        let mut heights = BTreeMap::new();
        for row in 0..rows {
            let height = area[row][col];

            let next_larger = max(heights.range(height..).map(|(_k, v)| *v)).unwrap_or(0);
            score.insert(
                (row, col),
                score[&(row, col)] * (row as i32 - next_larger as i32).abs(),
            );
            heights.insert(height, row);
        }
    }

    for row in 0..rows {
        let mut heights = BTreeMap::new();
        for col in (0..cols).rev() {
            let height = area[row][col];

            let next_larger = min(heights.range(height..).map(|(_k, v)| *v)).unwrap_or(cols - 1);
            score.insert(
                (row, col),
                score[&(row, col)] * (col as i32 - next_larger as i32).abs(),
            );
            heights.insert(height, col);
        }
    }

    for col in 0..cols {
        let mut heights = BTreeMap::new();
        for row in (0..rows).rev() {
            let height = area[row][col];

            let next_larger = min(heights.range(height..).map(|(_k, v)| *v)).unwrap_or(rows - 1);
            score.insert(
                (row, col),
                score[&(row, col)] * (row as i32 - next_larger as i32).abs(),
            );
            heights.insert(height, row);
        }
    }

    println!("{}", max(score.values()).unwrap());

    Ok(())
}
