// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
#![feature(linked_list_cursors)]

use anyhow::Result;
use itertools::Itertools;
use std::collections::LinkedList;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut list = LinkedList::new();

    for line in stdin.lock().lines() {
        let line = line?;
        let num: i32 = line.parse()?;
        list.push_back((num, false));
    }

    let len = list.len() as i32 - 1;
    let mut to_move = list.len();
    // let mut to_move = 7;

    let mut cur = list.cursor_front_mut();
    while to_move > 0 {
        to_move -= 1;
        loop {
            match cur.current() {
                Some((_num, false)) => {
                    let (num, _visited) = cur.remove_current().unwrap();
                    dbg!(num);
                    if cur.current().is_none() {
                        cur.move_next();
                    }
                    let shift = num % len;
                    let shift = if (shift - len).abs() < shift.abs() {
                        shift - len
                    } else if (shift + len).abs() < shift.abs() {
                        shift + len
                    } else {
                        shift
                    };
                    for _i in 0..shift {
                        cur.move_next();
                        if cur.current().is_none() {
                            cur.move_next();
                        }
                    }
                    for _i in shift..0 {
                        cur.move_prev();
                        if cur.current().is_none() {
                            cur.move_prev();
                        }
                    }
                    cur.insert_before((num, true));
                    for _i in 0..shift + 1 {
                        cur.move_prev();
                        if cur.current().is_none() {
                            cur.move_prev();
                        }
                    }
                    for _i in shift..0 {
                        cur.move_next();
                        if cur.current().is_none() {
                            cur.move_next();
                        }
                    }
                    break;
                }
                Some(_) | None => cur.move_next(),
            }
        }
    }
    // println!("{:?}", list.iter().cloned().map(|(n, _v)| n).collect_vec());
    let list = list.iter().map(|(elem, _v)| elem).cloned().collect_vec();
    let zero_index = list.iter().position(|elem|  *elem == 0).unwrap();
    let res: i32 = list[(zero_index + 1000) % list.len()]
        + list[(zero_index + 2000) % list.len()]
        + list[(zero_index + 3000) % list.len()];

    println!("{res}");
    Ok(())
}
