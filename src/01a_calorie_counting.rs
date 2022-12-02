// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use text_io::read;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    let mut max = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        }
        else {
            let n :i32 =  read!("{}", line.bytes());
            sum = sum + n;
        }
    }
    println!("{}", max);
    Ok(())
}
