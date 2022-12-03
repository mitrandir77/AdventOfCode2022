// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;
use std::collections::BTreeSet;
use itertools::Itertools;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for mut chunk in &stdin.lock().lines().chunks(3) {
        let a = BTreeSet::from_iter(chunk.next().unwrap()?.trim().bytes());
        let b = BTreeSet::from_iter(chunk.next().unwrap()?.trim().bytes());
        let c = BTreeSet::from_iter(chunk.next().unwrap()?.trim().bytes());
        let inter: BTreeSet<_> = a.intersection(&b).cloned().collect();
        let line_sum: i32 = inter.intersection(&c).map(
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
