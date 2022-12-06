// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeSet;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).expect("Couldn't read");
    let s: Vec<_> = s.chars().collect();
    for (i, w) in s[..].windows(14).enumerate() {
        let mut temp_set = BTreeSet::new();
        if w.iter().all(move |x| temp_set.insert(x)) {
            println!("{}", i + 14);
            break;
        }
    }

    Ok(())
}
