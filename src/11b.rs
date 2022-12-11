// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scan;
use std::collections::BTreeMap;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

struct Monkey {
    no: i64,
    operation: Box<dyn Fn(i64) -> i64>,
    divisable_by: i64,
    if_true: i64,
    if_false: i64,
    items: Vec<i64>,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            no: 0,
            operation: Box::new(|x| x),
            divisable_by: 0,
            if_true: 0,
            if_false: 0,
            items: Vec::new(),
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut monkeys = BTreeMap::new();
    let mut monkey = Monkey::new();
    let mut score = BTreeMap::new();
    let mut factor = 1;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            ("Monkey ", let monkey_no: i64, ":") => {
                monkey.no = monkey_no;
            },
            ("Starting items: ", [let arg: i64],*: Vec<i64> ) => {
                monkey.items = arg;
            },
            ("Operation: new = old ", let op: char, " old") => {
                monkey.operation = match op {
                    '+' => Box::new(|old| old + old),
                    '*' => Box::new(|old| old * old),
                    '-' => Box::new(|_old| 0),
                    '/' => Box::new(|_old| 1),
                    _ => panic!("bad operation"),
                };
            },
            ("Operation: new = old ", let op: char, " ", let arg:i64) => {
                monkey.operation = match op {
                    '+' => Box::new(move |old| old + arg),
                    '*' => Box::new(move |old| old * arg),
                    '-' => Box::new(move |old| old - arg),
                    '/' => Box::new(move |old| old / arg),
                    _ => panic!("bad operation"),
                };
            },
            ("Test: divisible by ", let divisible_by: i64) => {
                monkey.divisable_by = divisible_by;
                factor *= divisible_by;
            },
            ("If true: throw to monkey ", let if_true: i64) => {
                monkey.if_true = if_true;
            },
            ("If false: throw to monkey ", let if_false: i64) => {
                monkey.if_false= if_false;
            },
            ("") => {
                monkeys.insert(monkey.no, monkey);
                monkey = Monkey::new();
            }
        )
        .unwrap();
    }
    monkeys.insert(monkey.no, monkey);

    for _round in 0..10000 {
        let keys: Vec<i64> = monkeys.keys().cloned().collect();
        for key in keys {
            for item in monkeys[&key].items.clone() {
                *score.entry(key).or_insert(0) += 1;
                let mut item = (monkeys[&key].operation)(item);
                item = item % factor + factor;
                if item % monkeys[&key].divisable_by == 0 {
                    monkeys
                        .get_mut(&monkeys[&key].if_true.clone())
                        .unwrap()
                        .items
                        .push(item);
                } else {
                    monkeys
                        .get_mut(&monkeys[&key].if_false.clone())
                        .unwrap()
                        .items
                        .push(item);
                }
            }
            monkeys.get_mut(&key).unwrap().items.clear();
        }
    }
    let mut vals: Vec<_> = score.values().collect();
    vals.sort();
    let result: i64 = vals.pop().unwrap() * vals.pop().unwrap();
    println!("{}", result);
    Ok(())
}
