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

struct Map {
    tiles: Vec<Vec<Tile>>,
    xsize: i64,
    ysize: i64,
}

impl Map {
    fn get(&self, x: i64, y: i64) -> Tile {
        if (0..self.tiles.len()).contains(&(y as usize)) {
            if (0..self.tiles[y as usize].len()).contains(&(x as usize)) {
                self.tiles[y as usize][x as usize]
            } else {
                Tile::Missing
            }
        } else {
            Tile::Missing
        }
    }

    fn get_wrapping(&self, x: i64, y: i64) -> Tile {
        let new_x = (x + self.xsize) % self.xsize;
        let new_y = (y + self.ysize) % self.ysize;
        self.get(new_x, new_y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Foward(i64),
}

#[derive(Debug, Clone, Copy)]
struct Facing {
    x: i64,
    y: i64,
}

impl Facing {
    fn right() -> Self {
        Facing { x: 1, y: 0 }
    }

    fn turn_left(&mut self) {
        *self = Facing {
            x: self.y,
            y: -self.x,
        };
    }

    fn turn_right(&mut self) {
        *self = Facing {
            x: -self.y,
            y: self.x,
        };
    }

    fn as_number(&self) -> i64 {
        match self {
            Facing { x: 1, y: 0 } => 0,
            Facing { x: 0, y: 1 } => 1,
            Facing { x: -1, y: 0 } => 2,
            Facing { x: 0, y: -1 } => 3,
            _ => {
                panic!("wrong facing");
            }
        }
    }

    fn step(&self, x: i64, y: i64) -> (i64, i64) {
        let mx = x + self.x;
        let my = y + self.y;
        (mx, my)
    }
}

// Handles the trickest case of transitioning across cube edge.
// Walk along edges of flat cube until the edge that's ajacent in 3D is found.
fn walk(
    map: &Map,
    (mut x, mut y): (i64, i64),
    mut facing: Facing,
    turn_inside: fn(&mut Facing) -> (),
    turn_outside: fn(&mut Facing) -> (),
) -> Option<(Facing, (i64, i64))> {
    #[derive(PartialEq, Eq, Debug)]
    enum WalkAction {
        Corner,
        ReverseCorner,
        Forward,
    }
    use WalkAction::*;

    let mut x2 = x + facing.x;
    let mut y2 = y + facing.y;

    turn_inside(&mut facing);
    let mut actions = vec![];
    let mut reverse = None;
    for i in 0.. {
        let (nx, ny) = facing.step(x, y);
        let (nx2, ny2) = facing.step(x2, y2);

        if map.get(nx, ny) == Tile::Missing && map.get(nx2, ny2) == Tile::Missing {
            turn_inside(&mut facing);
            (x2, y2) = (nx, ny);
            actions.push(Corner);
        } else if map.get(nx, ny) != Tile::Missing && map.get(nx2, ny2) != Tile::Missing {
            turn_outside(&mut facing);
            (x, y) = (nx2, ny2);
            actions.push(ReverseCorner);
            reverse = Some(i);
        } else {
            (x, y) = (nx, ny);
            (x2, y2) = (nx2, ny2);
            actions.push(Forward);
        }

        if let Some(rev) = reverse {
            let diff = rev - i;

            if actions[rev - diff] == Corner && actions[rev + diff] == Corner {
                return None;
            }
            if i == 2 * rev {
                turn_inside(&mut facing);
                return Some((facing, (x, y)));
            }
        }
    }
    return None;
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    // Parse map.
    let mut tiles = vec![];
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
        tiles.push(row);
    }
    let ysize = tiles.len() as i64;
    let map = Map {
        tiles,
        ysize,
        xsize,
    };

    // Parse directions
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

    // Find starting point.
    let (mut x, mut y) = (0, 0);
    while map.get(x, y) != Tile::Open {
        x += 1;
    }
    let mut facing = Facing::right();

    // Follow directions.
    for d in directions {
        match d {
            Direction::Left => {
                facing.turn_left();
            }
            Direction::Right => {
                facing.turn_right();
            }
            Direction::Foward(mut steps) => loop {
                let (mut new_x, mut new_y) = facing.step(x, y);
                let mut new_facing = facing;

                let mut tile = if (0..=xsize).contains(&new_x) && (0..=ysize).contains(&new_y) {
                    map.get(new_x, new_y)
                } else if !(0..=xsize).contains(&new_x) && xsize % 4 == 0 {
                    // Special case for maps that have 4 sides flat in a row
                    map.get_wrapping(new_x, new_y)
                } else if !(0..=ysize).contains(&new_y) && ysize % 4 == 0 {
                    // Special case for maps that have 4 sides flat in a column
                    map.get_wrapping(new_x, new_y)
                } else {
                    Tile::Missing
                };

                if tile == Tile::Missing {
                    let counter = walk(&map, (x, y), facing, Facing::turn_left, Facing::turn_right);
                    let clock = walk(&map, (x, y), facing, Facing::turn_right, Facing::turn_left);

                    if counter.is_some() && clock.is_some() {
                        panic!("both walks worked!");
                    }
                    (new_facing, (new_x, new_y)) = if let Some(res) = counter {
                        res
                    } else if let Some(res) = clock {
                        res
                    } else {
                        panic!("neither walk worked!");
                    };
                    tile = map.get(new_x, new_y);
                }

                if tile == Tile::Rock {
                    break;
                }
                if tile == Tile::Open {
                    x = new_x;
                    y = new_y;
                    facing = new_facing;
                    steps -= 1;
                    if steps == 0 {
                        break;
                    }
                }
            },
        }
    }

    let res = 1000 * (y + 1) + 4 * (x + 1) + facing.as_number();
    dbg!(facing, x, y);
    println!("{res}");
    Ok(())
}
