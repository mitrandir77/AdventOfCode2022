// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;
use text_io::read;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    let mut elves = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            let n: i32 = read!("{}", line.bytes());
            sum = sum + n;
        }
    }
    elves.push(sum);
    elves.sort();
    let top_3_sum: i32 = elves[elves.len() - 3..elves.len()].iter().sum();
    println!("{}", top_3_sum);
    Ok(())
}
