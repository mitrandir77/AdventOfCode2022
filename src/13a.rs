// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use json::JsonValue;
use std::io::BufRead;

fn compare(first: &JsonValue, second: &JsonValue) -> Option<bool> {
    match (first, second) {
        (JsonValue::Number(a), JsonValue::Number(b)) => {
            let a: f64 = (*a).into();
            let b: f64 = (*b).into();

            if a != b {
                return Some(a < b);
            }
        }
        (JsonValue::Array(a), JsonValue::Array(b)) => {
            for pair in a.iter().zip_longest(b) {
                match pair {
                    Both(l, r) => {
                        if let Some(res) = compare(l, r) {
                            return Some(res);
                        }
                    }
                    Left(_l) => return Some(false),
                    Right(_r) => return Some(true),
                }
            }
        }
        (JsonValue::Array(a), JsonValue::Number(b)) => {
            if let Some(res) = compare(
                &JsonValue::Array(a.to_vec()),
                &JsonValue::Array(vec![JsonValue::Number(*b)]),
            ) {
                return Some(res);
            }
        }
        (JsonValue::Number(a), JsonValue::Array(b)) => {
            if let Some(res) = compare(
                &JsonValue::Array(vec![JsonValue::Number(*a)]),
                &JsonValue::Array(b.to_vec()),
            ) {
                return Some(res);
            }
        }
        (a, b) => {
            panic!("unexpected value {} {}", a, b);
        }
    }
    None
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for (i, pair) in (stdin.lock().lines().chunks(3).into_iter()).enumerate() {
        let pair: Vec<_> = pair.collect();

        let first = pair[0].as_ref().unwrap().to_owned();
        let first = json::parse(&first)?;
        let second = pair[1].as_ref().unwrap().to_owned();
        let second = json::parse(&second)?;

        if compare(&first, &second).unwrap() {
            sum += i + 1;
        }
    }
    println!("{}", sum);
    Ok(())
}
