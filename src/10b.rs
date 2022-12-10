// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use scan_rules::scan;
use std::io::BufRead;
#[macro_use]
extern crate scan_rules;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut cycle_no = 1;
    let mut x = 1;

    let render = |cycle_no: i32, x: i32| {
        if (cycle_no - 1) % 40 == 0 {
            println!();
        }
        if (((cycle_no - 1) % 40) - x).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    };

    render(cycle_no, x);
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        scan!(&line;
            ("noop") => {
                cycle_no += 1;
                render(cycle_no, x);
            },
            ("addx", let arg: i32) => {
                cycle_no += 1;
                render(cycle_no, x);
                cycle_no += 1;
                x += arg;
                render(cycle_no, x);
            },
        )
        .unwrap();
    }
    Ok(())
}
