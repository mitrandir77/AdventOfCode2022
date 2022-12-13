// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use json::JsonValue;
use std::cmp::Ordering;
use std::io::BufRead;

fn compare(first: &JsonValue, second: &JsonValue) -> Ordering {
    match (first, second) {
        (JsonValue::Number(a), JsonValue::Number(b)) => {
            let a: f64 = (*a).into();
            let b: f64 = (*b).into();

            if a != b {
                return a.total_cmp(&b);
            }
        }
        (JsonValue::Array(a), JsonValue::Array(b)) => {
            for pair in a.iter().zip_longest(b) {
                match pair {
                    Both(l, r) => {
                        let res = compare(l, r);
                        if res.is_ne() {
                            return res;
                        }
                    }
                    Left(_l) => return Ordering::Greater,
                    Right(_r) => return Ordering::Less,
                }
            }
        }
        (JsonValue::Array(a), JsonValue::Number(b)) => {
            let res = compare(
                &JsonValue::Array(a.to_vec()),
                &JsonValue::Array(vec![JsonValue::Number(*b)]),
            );
            if res.is_ne() {
                return res;
            }
        }
        (JsonValue::Number(a), JsonValue::Array(b)) => {
            let res = compare(
                &JsonValue::Array(vec![JsonValue::Number(*a)]),
                &JsonValue::Array(b.to_vec()),
            );
            if res.is_ne() {
                return res;
            }
        }
        (a, b) => {
            panic!("unexpected value {} {}", a, b);
        }
    }
    Ordering::Equal
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut packets = vec![];
    for pair in stdin.lock().lines().chunks(3).into_iter() {
        let pair: Vec<_> = pair.collect();

        let first = pair[0].as_ref().unwrap().to_owned();
        let first = json::parse(&first)?;
        let second = pair[1].as_ref().unwrap().to_owned();
        let second = json::parse(&second)?;

        packets.push(first);
        packets.push(second);
    }
    let div_a = json::parse("[[2]]")?;
    let div_b = json::parse("[[6]]")?;
    packets.push(div_a.clone());
    packets.push(div_b.clone());
    packets.sort_by(compare);

    let mut i_a = 0;
    let mut i_b = 0;
    for (i, p) in packets.iter().enumerate() {
        if p == &div_a {
            i_a = i + 1;
        }
        if p == &div_b {
            i_b = i + 1;
        }
    }
    println!("{}", i_a * i_b);
    Ok(())
}
