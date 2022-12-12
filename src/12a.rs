// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut area = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let chars: Vec<char> = line.trim().chars().collect();
        area.push(chars);
    }

    let rows = area.len();
    let cols = area[0].len();

    let mut start = None;
    let mut end = None;
    for row in 0..rows {
        for col in 0..cols {
            if area[row][col] == 'S' {
                start = Some((row, col));
            }
            if area[row][col] == 'E' {
                end = Some((row, col));
            }
        }
    }
    let start = if let Some(start) = start {
        (start.0 as i32, start.1 as i32)
    } else {
        panic!("No start");
    };
    let _end = if let Some(end) = end {
        end
    } else {
        panic!("No end");
    };

    let mut queue = VecDeque::new();
    let mut seen = BTreeSet::new();

    queue.push_back((start, 0));
    seen.insert(start);

    let mut result = None;

    while let Some(((x, y), dist)) = queue.pop_front() {
        let mut val = area[x as usize][y as usize];
        if val == 'S' {
            val = 'a';
        }
        if val == 'E' {
            result = Some(dist);
            break;
        }
        for (next_x, next_y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if next_x < 0 || next_y < 0 {
                continue;
            }
            if let Some(row) = area.get(next_x as usize) {
                if let Some(mut next_val) = row.get(next_y as usize).cloned() {
                    if next_val == 'E' {
                        next_val = 'z';
                    }
                    if (next_val as i32 - val as i32) <= 1 && !seen.contains(&(next_x, next_y)) {
                        seen.insert((next_x, next_y));
                        queue.push_back(((next_x, next_y), dist+1));
                    }
                }
            }
        }
    }

    println!("{}", result.unwrap());

    Ok(())
}
