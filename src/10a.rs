// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;
use scan_rules::scan;
#[macro_use] extern crate scan_rules;


fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut cycle_no = 1;
    let mut x = 1;
    let mut result = 0;
    let log_moments = vec![20, 60, 100, 140, 180, 220];

    let mut maybe_log = |cycle_no: i32, x: i32| {
        if log_moments.contains(&cycle_no) {
            result += cycle_no * x;
        }
    };


    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            ("noop") => {
                cycle_no += 1;
                maybe_log(cycle_no, x);
            },
            ("addx", let arg: i32) => {
                cycle_no += 1;
                maybe_log(cycle_no, x);
                cycle_no += 1;
                x += arg;
                maybe_log(cycle_no, x);
            },
        ).unwrap();

    }
    println!("{}", result);
    Ok(())
}
