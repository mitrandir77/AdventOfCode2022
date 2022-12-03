// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?.trim().to_owned();
        let len = line.len();
        let a = BTreeSet::from_iter(line[0..len/2].bytes());
        let b = BTreeSet::from_iter(line[len/2..len].bytes());
        let line_sum: i32 = a.intersection(&b).map(
            |c| if c.is_ascii_lowercase() {
                *c as i32 - b'a' as i32 + 1
            } else if c.is_ascii_uppercase(){
                *c  as i32- b'A' as i32 + 27
            } else {
                0
            }
        ).sum::<i32>();
        sum = sum + line_sum;
    }
    println!("{}", sum);
    Ok(())
}
