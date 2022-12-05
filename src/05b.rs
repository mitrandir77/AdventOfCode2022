// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::io::BufRead;
use text_io::try_scan;

fn read_move() -> Result<(usize, usize, usize)> {
    let howmany: usize;
    let from: usize;
    let to: usize;
    try_scan!("move {} from {} to {}", howmany, from, to);
    Ok((howmany, from, to))
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stacks: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                stacks
                    .entry(i / 4 + 1)
                    .or_insert_with(|| VecDeque::new())
                    .push_front(c);
            }
        }
    }
    while let Ok((howmany, from, to)) = read_move() {
        let mut temp = Vec::new();
        for _ in 0..howmany {
            temp.push(
                stacks
                    .entry(from)
                    .or_insert_with(|| VecDeque::new())
                    .pop_back()
                    .unwrap(),
            );
        }
        for item in (temp.into_iter()).rev() {
            stacks
                .entry(to)
                .or_insert_with(|| VecDeque::new())
                .push_back(item);
        }
    }
    for (_, stack) in stacks {
        if let Some(c) = stack.back() {
            print!("{}", c);
        }
    }
    println!();
    Ok(())
}
