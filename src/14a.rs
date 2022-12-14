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
    let mut area = [[Air; 501]; 1010];
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;

        let mut prev = None;
        for elem in line.split(" -> ") {
            let (x2, y2) = scan!(elem;  (let x: usize, ",", let y: usize) => (x,y)).unwrap();

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

    let mut steps = 0;
    'outer: loop {
        let (mut x, mut y) = (500, 0);

        if area[x][y] != Air {
            panic!("GAME OVER!");
        }

        loop {
            if area[x][y + 1] == Air {
                y += 1;
            } else if area[x - 1][y + 1] == Air {
                x -= 1;
                y += 1;
            } else if area[x + 1][y + 1] == Air {
                x += 1;
                y += 1;
            } else {
                area[x][y] = Sand;
                break;
            }

            if y == 500 {
                println!("{}", steps);
                break 'outer;
            }
        }
        steps += 1;
    }
    Ok(())
}
