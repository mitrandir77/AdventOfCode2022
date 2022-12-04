// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::ops::RangeInclusive;
use text_io::try_scan;

fn read_pair() -> Result<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let a1: i32;
    let a2: i32;
    let b1: i32;
    let b2: i32;
    try_scan!("{}-{},{}-{}", a1, a2, b1, b2);
    Ok((a1..=a2, b1..=b2))
}

fn main() -> Result<()> {
    let mut sum = 0;
    while let Ok((a, b)) = read_pair() {
        if (a.contains(b.start()) || a.contains(b.end()))
            || (b.contains(a.start()) || b.contains(a.end()))
        {
            sum = sum + 1;
        }
    }
    println!("{}", sum);
    Ok(())
}
