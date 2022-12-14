// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::cmp::max;
use std::cmp::min;
use std::io::BufRead;

#[macro_use]
extern crate scan_rules;

#[derive(PartialEq, Clone, Copy)]
enum Point {
    Rock,
    Air,
    Sand,
}

use Point::*;

fn main() -> Result<()> {
    let mut area = [[Air; 503]; 1010];
    let stdin = std::io::stdin();
    let mut y_max = 0;
    for line in stdin.lock().lines() {
        let line = line?;

        let mut prev = None;
        for elem in line.split(" -> ") {
            let (x2, y2) = scan!(elem;  (let x: usize, ",", let y: usize) => (x,y)).unwrap();
            y_max = max(y2, y_max);

            if let Some((x1, y1)) = prev {
                if x2 == x1 {
                    for y in min(y1, y2)..=max(y1, y2) {
                        area[x1][y] = Rock;
                    }
                } else if y2 == y1 {
                    for x in min(x1, x2)..=max(x1, x2) {
                        area[x][y1] = Rock;
                    }
                }
            };
            prev = Some((x2, y2))
        }
    }
    y_max += 2;

    let mut steps = 0;
    'outer: loop {
        let (mut x, mut y) = (500, 0);

        if area[x][y] != Air {
            println!("{}", steps);
            break 'outer;
        }

        loop {
            if area[x][y + 1] == Air && y + 1 != y_max {
                y += 1;
            } else if area[x - 1][y + 1] == Air && y + 1 != y_max {
                x -= 1;
                y += 1;
            } else if area[x + 1][y + 1] == Air && y + 1 != y_max {
                x += 1;
                y += 1;
            } else {
                area[x][y] = Sand;
                break;
            }
        }
        steps += 1;
    }
    Ok(())
}
