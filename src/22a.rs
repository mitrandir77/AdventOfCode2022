// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
#![feature(iter_intersperse)]

use anyhow::Result;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Open,
    Missing,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Foward(i64),
}

#[derive(Debug, Clone, Copy)]
struct Facing(i8, i8);

impl Facing {
    fn right() -> Self {
        Facing(1, 0)
    }

    fn x(&self) -> i64 {
        self.0 as i64
    }

    fn y(&self) -> i64 {
        self.1 as i64
    }

    fn turn_left(&mut self) {
        *self = match self {
            Facing(1, 0) => Facing(0, -1),
            Facing(0, -1) => Facing(-1, 0),
            Facing(-1, 0) => Facing(0, 1),
            Facing(0, 1) => Facing(1, 0),
            _ => {
                panic!("unexpected step");
            }
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Facing(1, 0) => Facing(0, 1),
            Facing(0, 1) => Facing(-1, 0),
            Facing(-1, 0) => Facing(0, -1),
            Facing(0, -1) => Facing(1, 0),
            _ => {
                panic!("unexpected step");
            }
        };
    }

    fn as_number(&self) -> usize {
        match self {
            Facing(1, 0) => 0,
            Facing(0, 1) => 1,
            Facing(-1, 0) => 2,
            Facing(0, -1) => 3,
            _ => {
                panic!("wrong facing");
            }
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut map = vec![];
    let mut xsize = 0;
    for line in stdin.lock().lines() {
        let mut row = vec![];
        let line = line?;
        if line.trim() == "" {
            break;
        }
        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Rock,
                '.' => Tile::Open,
                ' ' => Tile::Missing,
                _ => {
                    panic!("unexpected char");
                }
            };
            row.push(tile);
        }
        xsize = xsize.max(row.len() as i64);
        map.push(row);
    }
    let ysize = map.len() as i64;

    let mut directions = vec![];
    for line in stdin.lock().lines() {
        let mut num = 0;
        for c in line?.chars() {
            match c {
                'L' => {
                    if num > 0 {
                        directions.push(Direction::Foward(num));
                        num = 0;
                    }
                    directions.push(Direction::Left);
                }
                'R' => {
                    if num > 0 {
                        directions.push(Direction::Foward(num));
                        num = 0;
                    }
                    directions.push(Direction::Right);
                }
                c if c.is_numeric() => {
                    let digit: i64 = c.to_digit(10).unwrap() as i64;
                    num = num * 10 + digit;
                }
                _ => {
                    panic!("unexpected char");
                }
            }
        }
        if num > 0 {
            directions.push(Direction::Foward(num));
        }
    }

    let mut y: usize = 0;
    let mut x: usize = 0;
    while map[y][x] != Tile::Open {
        x += 1;
    }
    let mut facing = Facing::right();
    for d in directions {
        let (mut cur_x, mut cur_y) = (x, y);
        match d {
            Direction::Left => {
                facing.turn_left();
            }
            Direction::Right => {
                facing.turn_right();
            }
            Direction::Foward(mut steps) => loop {
                let new_x = ((cur_x as i64 + facing.x() + xsize) % xsize) as usize;
                let new_y = ((cur_y as i64 + facing.y() + ysize) % ysize) as usize;
                // println!("{cur_x}, {cur_y}, {facing}");
                match map[new_y].get(new_x).unwrap_or(&Tile::Missing) {
                    Tile::Missing => {}
                    Tile::Rock => {
                        break;
                    }
                    Tile::Open => {
                        x = new_x;
                        y = new_y;
                        steps -= 1;
                        if steps == 0 {
                            break;
                        }
                    }
                }
                cur_x = new_x;
                cur_y = new_y;
            },
        }
    }

    let res = 1000 * (y + 1) + 4 * (x + 1) + facing.as_number();
    println!("{res}");
    Ok(())
}
