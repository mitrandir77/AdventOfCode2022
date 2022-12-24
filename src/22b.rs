// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
use anyhow::Result;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Open,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}


struct Map {
    tiles: Vec<Vec<Tile>>,
    xsize: i64,
    ysize: i64,
}

impl Map {
    fn get(&self, p: Point) -> Tile {
        if (0..self.tiles.len()).contains(&(p.y as usize)) {
            if (0..self.tiles[p.y as usize].len()).contains(&(p.x as usize)) {
                self.tiles[p.y as usize][p.x as usize]
            } else {
                Tile::Missing
            }
        } else {
            Tile::Missing
        }
    }

    fn get_wrapping(&self, p: Point) -> Tile {
        self.get(Point {
            x: (p.x + self.xsize) % self.xsize,
            y: (p.y + self.ysize) % self.ysize,
        })
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

    fn step(&self, p: Point) -> Point {
        Point::new(p.x + self.x, p.y + self.y)
    }
}

// Handles the trickest case of transitioning across cube edge.
// Walk along edges of flat cube until the edge that's ajacent in 3D is found.
fn walk(
    map: &Map,
    mut p_inside: Point,
    mut facing: Facing,
    turn_inside: fn(&mut Facing) -> (),
    turn_outside: fn(&mut Facing) -> (),
) -> Option<(Facing, Point)> {
    #[derive(PartialEq, Eq, Debug)]
    enum WalkAction {
        Corner,
        ReverseCorner,
        Forward,
    }
    use WalkAction::*;

    let mut p_outside = facing.step(p_inside);

    turn_inside(&mut facing);
    let mut actions = vec![];
    let mut reverse = None;
    for i in 0.. {
        let new_p_inside = facing.step(p_inside);
        let new_p_outside = facing.step(p_outside);

        if map.get(new_p_inside) == Tile::Missing && map.get(new_p_outside) == Tile::Missing {
            turn_inside(&mut facing);
            p_outside = new_p_inside;
            actions.push(Corner);
        } else if map.get(new_p_inside) != Tile::Missing && map.get(new_p_outside) != Tile::Missing
        {
            turn_outside(&mut facing);
            p_inside = new_p_outside;
            actions.push(ReverseCorner);
            reverse = Some(i);
        } else {
            p_inside = new_p_inside;
            p_outside = new_p_outside;
            actions.push(Forward);
        }

        if let Some(rev) = reverse {
            let diff = rev - i;

            if actions[rev - diff] == Corner && actions[rev + diff] == Corner {
                return None;
            }
            if i == 2 * rev {
                turn_inside(&mut facing);
                return Some((facing, p_inside));
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
    let mut p = Point::new(0,0);
    while map.get(p) != Tile::Open {
        p.x += 1;
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
                let mut new = facing.step(p);
                let mut new_facing = facing;

                let mut tile = if (0..=xsize).contains(&new.x) && (0..=ysize).contains(&new.y) {
                    map.get(new)
                } else if !(0..=xsize).contains(&new.x) && xsize % 4 == 0 {
                    // Special case for maps that have 4 sides flat in a row
                    map.get_wrapping(new)
                } else if !(0..=ysize).contains(&new.y) && ysize % 4 == 0 {
                    // Special case for maps that have 4 sides flat in a column
                    map.get_wrapping(new)
                } else {
                    Tile::Missing
                };

                if tile == Tile::Missing {
                    let counter = walk(&map, p, facing, Facing::turn_left, Facing::turn_right);
                    let clock = walk(&map, p, facing, Facing::turn_right, Facing::turn_left);

                    if counter.is_some() && clock.is_some() {
                        panic!("both walks worked!");
                    }
                    (new_facing, new) = if let Some(res) = counter {
                        res
                    } else if let Some(res) = clock {
                        res
                    } else {
                        panic!("neither walk worked!");
                    };
                    tile = map.get(new);
                }

                if tile == Tile::Rock {
                    break;
                }
                if tile == Tile::Open {
                    p = new;
                    facing = new_facing;
                    steps -= 1;
                    if steps == 0 {
                        break;
                    }
                }
            },
        }
    }

    let res = 1000 * (p.y + 1) + 4 * (p.x + 1) + facing.as_number();
    println!("{res}");
    Ok(())
}
