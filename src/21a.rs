// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scan;
use scan_rules::scanner::Word;
use std::collections::HashMap;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

#[derive(Debug, Clone)]
enum Monkey {
    Leaf(i64),
    Op(char, String, String),
}

fn dfs(monkey_id: &str, monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = monkeys.get(monkey_id).unwrap();

    match monkey {
        Monkey::Leaf(val) => *val,
        Monkey::Op(op, first_id, second_id) => {
            let first_val = dfs(first_id, monkeys);
            let second_val = dfs(&second_id, monkeys);
            match op {
                '+' => first_val + second_val,
                '*' => first_val * second_val,
                '-' => first_val - second_val,
                '/' => first_val / second_val,
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
            (let monkey_id: Word, ": ", let value: i64) => {
                monkeys.insert(monkey_id.to_string(), Monkey::Leaf(value));
            },
            (let monkey_id: Word, ": ", let first_id: Word, " ", let op: char, " ", let second_id: Word) => {
                monkeys.insert(monkey_id.to_string(), Monkey::Op(op, first_id.to_string(), second_id.to_string()));
            }
        )
        .unwrap();
    }

    let res = dfs("root", &monkeys);

    println!("{res}");

    Ok(())
}
