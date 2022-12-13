// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::io::BufRead;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => {
                if a != b {
                    return a.cmp(b);
                }
            }
            (Packet::List(a), Packet::List(b)) => {
                for pair in a.iter().zip_longest(b) {
                    match pair {
                        Both(l, r) => {
                            let res = l.cmp(r);
                            if res.is_ne() {
                                return res;
                            }
                        }
                        Left(_l) => return Ordering::Greater,
                        Right(_r) => return Ordering::Less,
                    }
                }
            }
            (al @ Packet::List(_a), Packet::Number(b)) => {
                let res = al.cmp(&Packet::List(vec![Packet::Number(*b)]));
                if res.is_ne() {
                    return res;
                }
            }
            (Packet::Number(a), bl @ Packet::List(_b)) => {
                let res = Packet::List(vec![Packet::Number(*a)]).cmp(bl);
                if res.is_ne() {
                    return res;
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut packets = vec![];
    for pair in stdin.lock().lines().chunks(3).into_iter() {
        let pair: Vec<_> = pair.collect();

        let first: Packet = serde_json::from_str(pair[0].as_ref().unwrap())?;
        let second: Packet = serde_json::from_str(pair[1].as_ref().unwrap())?;

        packets.push(first);
        packets.push(second);
    }
    let div_a = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let div_b = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(div_a.clone());
    packets.push(div_b.clone());
    packets.sort();

    let i_a = packets.iter().position(|p| p == &div_a).unwrap() + 1;
    let i_b = packets.iter().position(|p| p == &div_b).unwrap() + 1;
    println!("{}", i_a * i_b);
    Ok(())
}
