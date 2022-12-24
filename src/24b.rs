// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
use anyhow::Result;
use multimap::MultiMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn moves(&self) -> [Point; 5] {
        [
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Blizzard {
    x: i8,
    y: i8,
}

impl Blizzard {
    fn right() -> Self {
        Self { x: 1, y: 0 }
    }

    fn left() -> Self {
        Self { x: -1, y: 0 }
    }

    fn up() -> Self {
        Self { x: 0, y: -1 }
    }

    fn down() -> Self {
        Self { x: 0, y: 1 }
    }

    fn step(&self, map: &Map, p: Point) -> Point {
        Point {
            x: (p.x + (self.x as i64) - 1 + map.xsize - 2) % (map.xsize - 2) + 1,
            y: (p.y + (self.y as i64) - 1 + map.ysize - 2) % (map.ysize - 2) + 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Blizzard(Blizzard),
}

struct Map {
    tiles: MultiMap<Point, Tile>,
    xsize: i64,
    ysize: i64,
}

impl Map {
    fn get(&self, p: &Point) -> Option<&Tile> {
        self.tiles.get(p)
    }

    fn contains(&self, p: &Point) -> bool {
        self.tiles.contains_key(p)
    }

    fn tick(&mut self) {
        let mut new_tiles = MultiMap::new();

        for (p, vals) in self.tiles.iter_all() {
            for tile in vals {
                match tile {
                    t @ Tile::Rock => new_tiles.insert(*p, *t),
                    t @ Tile::Blizzard(blizzard) => {
                        let next = blizzard.step(&self, *p);
                        new_tiles.insert(next, *t)
                    }
                }
            }
        }
        self.tiles = new_tiles;
    }

    // fn debug_print(&self) {
    //     println!();
    //     for y in 0..self.ysize {
    //         for x in 0..self.xsize {
    //             match self.tiles.get_vec(&Point::new(x, y)) {
    //                 None => {
    //                     print!(".");
    //                 }
    //                 Some(vec) => {
    //                     if vec.len() == 1 {
    //                         match vec.first().unwrap() {
    //                             Tile::Rock => {
    //                                 print!("#");
    //                             }
    //                             Tile::Blizzard(Blizzard { x: 1, y: 0 }) => {
    //                                 print!(">");
    //                             }
    //                             Tile::Blizzard(Blizzard { x: -1, y: 0 }) => {
    //                                 print!("<");
    //                             }
    //                             Tile::Blizzard(Blizzard { x: 0, y: 1 }) => {
    //                                 print!("v");
    //                             }
    //                             Tile::Blizzard(Blizzard { x: 0, y: -1 }) => {
    //                                 print!("^");
    //                             }
    //                             _ => {
    //                                 panic!("unexpected blizzard");
    //                             }
    //                         }
    //                     } else {
    //                         print!("{}", vec.len());
    //                     }
    //                 }
    //             }
    //         }
    //         println!();
    //     }
    // }
}

fn bfs(start: Point, goal_y: i64, map: &mut Map) -> i64 {
    let mut queue = VecDeque::from([(start, 0)]);

    let mut last_dist = 0;
    let mut vis = HashSet::new();
    while let Some((p, dist)) = queue.pop_front() {
        if dist != last_dist {
            map.tick();
        }
        if p.y == goal_y {
            return dist;
        }
        for next in p.moves() {
            if !vis.contains(&(next, dist + 1)) && !map.contains(&next) && next.y >= 0 && next.y < map.ysize {
                queue.push_back((next, dist + 1));
                vis.insert((next, dist + 1));
            }
        }
        last_dist = dist;
    }
    panic!("no way to the goal");
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    // Parse map.
    let mut xsize = 0;
    let mut ysize = 0;
    let mut tiles = MultiMap::new();
    for (y, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        if line.trim() == "" {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Some(Tile::Rock),
                '.' => None,
                'v' => Some(Tile::Blizzard(Blizzard::down())),
                '^' => Some(Tile::Blizzard(Blizzard::up())),
                '>' => Some(Tile::Blizzard(Blizzard::right())),
                '<' => Some(Tile::Blizzard(Blizzard::left())),
                _ => {
                    panic!("unexpected char");
                }
            };
            if let Some(tile) = tile {
                tiles.insert(Point::new(x as i64, y as i64), tile);
            }
            xsize = xsize.max(x as i64);
            ysize = ysize.max(y as i64);
        }
    }
    xsize += 1;
    ysize += 1;
    let mut map = Map {
        tiles,
        ysize,
        xsize,
    };

    let mut start = Point::new(0, 0);
    loop {
        if map.get(&start) != Some(&Tile::Rock) {
            break;
        }
        start.x += 1;
    }
    let mut end = Point::new(0, map.ysize - 1);
    loop {
        if map.get(&end) != Some(&Tile::Rock) {
            break;
        }
        end.x += 1;
    }

    map.tick();
    let there = bfs(start, map.ysize - 1, &mut map);
    let back = bfs(end, 0, &mut map);
    let there_again = bfs(start, map.ysize - 1, &mut map);
    let dist = there + back + there_again;
    println!("{dist}");
    Ok(())
}
