// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scan;
use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

const EPS: f64 = 0.00001;

#[derive(Debug, Clone)]
enum Monkey {
    Leaf(f64),
    Op(char, String, String),
}

fn dfs(monkey_id: &str, monkeys: &HashMap<String, Monkey>) -> (f64, f64) {
    if monkey_id == "humn" {
        return (0.0, 1.0);
    }

    let monkey = monkeys.get(monkey_id).unwrap();

    match monkey {
        Monkey::Leaf(val) => (*val, 0.0),
        Monkey::Op(op, first_id, second_id) => {
            let (first_val, first_me) = dfs(first_id, monkeys);
            let (second_val, second_me) = dfs(&second_id, monkeys);
            match op {
                '+' => (first_val + second_val, first_me + second_me),
                '*' => (
                    first_val * second_val,
                    first_me * second_val + second_me * first_val,
                ),
                '-' => (first_val - second_val, first_me - second_me),
                '/' => (
                    first_val / second_val,
                    first_me / second_val + second_me / first_val,
                ),
                _ => panic!("unsupported op"),
            }
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            (let monkey_id: Word, ": ", let value: f64) => {
                monkeys.insert(monkey_id.to_string(), Monkey::Leaf(value));
            },
            (let monkey_id: Word, ": ", let first_id: Word, " ", let op: char, " ", let second_id: Word) => {
                monkeys.insert(monkey_id.to_string(), Monkey::Op(op, first_id.to_string(), second_id.to_string()));
            }
        )
        .unwrap();
    }

    let mut res = 0.0;
    if let Monkey::Op(_op, first_id, second_id) = &monkeys["root"] {
        let (first_val, first_me) = dfs(first_id, &monkeys);
        let (second_val, second_me) = dfs(second_id, &monkeys);

        match (first_me, second_me) {
            (val, zero) if zero.abs() < EPS => {
                res = (second_val - first_val) / val;
            }
            (zero, val) if zero.abs() < EPS => {
                res = (first_val - second_val) / val;
            }
            (_val1, _val2) => {
                panic!("human in both branches");
            }
        }
    }
    let ires = res as i64;

    println!("{ires}");

    Ok(())
}
