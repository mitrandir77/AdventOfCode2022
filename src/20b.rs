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
    let mut ids = vec![];

    for (id, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        let num: i64 = line.parse()?;
        list.push_back((id, num * 811589153));
        ids.push(id)
    }

    let len = list.len() as i64 - 1;

    for _mix in 0..10 {
        let mut cur = list.cursor_front_mut();
        for next_id in ids.iter() {
            loop {
                match cur.current() {
                    Some((id, _num)) if *id == *next_id => {
                        let (id, num) = cur.remove_current().unwrap();
                        // dbg!(num);
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
                        cur.insert_before((id, num));
                        break;
                    }
                    Some(_) | None => cur.move_next(),
                }
            }
        }
    }
    // println!("{:?}", list.iter().cloned().map(|(n, _v)| n).collect_vec());
    let list = list.iter().map(|(_id, elem)| elem).cloned().collect_vec();
    let zero_index = list.iter().position(|elem| *elem == 0).unwrap();
    let res: i64 = list[(zero_index + 1000) % list.len()]
        + list[(zero_index + 2000) % list.len()]
        + list[(zero_index + 3000) % list.len()];

    println!("{res}");
    Ok(())
}
