// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Shape(Vec<u8>);

impl Shape {
    fn from_str(spec: &str) -> Result<Vec<Self>> {
        let mut shapes = vec![];
        let mut rows: Vec<u8> = vec![];
        for line in spec.lines() {
            if line.trim() == "" && !rows.is_empty() {
                rows.reverse();
                shapes.push(Shape(rows));
                rows = Vec::new();
                continue;
            }
            rows.push(
                line.chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| match c {
                        '#' => 1 << i,
                        '.' => 0,
                        _ => {
                            panic!("wrong char");
                        }
                    })
                    .sum(),
            );
        }
        if !rows.is_empty() {
            rows.reverse();
            shapes.push(Shape(rows));
        }
        Ok(shapes)
    }
}

struct Rock {
    shape: Shape,
    pos: usize,
}

impl Rock {
    // pushes rock in one of two directions
    fn push(&mut self, map: &Vec<u8>, dir: Direction) {
        let mut new_shape = self.shape.clone();
        match dir {
            Direction::Left => {
                for bitmask in new_shape.0.iter_mut() {
                    // 7th bit on
                    if *bitmask >= 64 {
                        return;
                    }
                    *bitmask <<= 1;
                }
            }
            Direction::Right => {
                for bitmask in new_shape.0.iter_mut() {
                    // 1st bit on
                    if *bitmask % 2 == 1 {
                        return;
                    }
                    *bitmask >>= 1;
                }
            }
        }
        
        let cand = Rock {
            shape: new_shape,
            pos: self.pos,
        };
        if cand.collides(map) {
            return;
        }
        *self= cand;
    }

    // simulates rock falling by one unit down
    fn fall(&mut self, map: &mut Vec<u8>) -> bool {
        if self.pos == 0 {
            self.freeze(map);
            return false;
        }

        self.pos -= 1;

        if !self.collides(map) {
            return true;
        }

        self.pos += 1;
        self.freeze(map);
        false
    }

    // makes the rock pernament part of the map
    fn freeze(&mut self, map: &mut Vec<u8>) {
        while map.len() < self.pos + self.shape.0.len() {
            map.push(0);
        }
        for (i, bitmask) in self.shape.0.iter().enumerate() {
            map[self.pos + i] |= bitmask;
        }
    }

    // check if rock collides with anything on map
    fn collides(&self, map: &Vec<u8>) -> bool {
        for (i, bitmask) in self.shape.0.iter().enumerate() {
            if bitmask & map.get(self.pos + i).unwrap_or(&0) > 0 {
                return true;
            }
        }
        false
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut input = vec![];
    for line in stdin.lock().lines() {
        for c in line?.chars() {
            let dir = match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => {
                    panic!("unexpected char");
                }
            };
            input.push(dir);
        }
    }
    let shapes = Shape::from_str(
        "\
..####.

...#...
..###..
...#...

....#..
....#..
..###..

..#....
..#....
..#....
..#....

..##...
..##...",
    )?;

    let mut cur_shape_id = 0;
    let mut map = Vec::new();
    let mut rock = Rock {
        shape: shapes[cur_shape_id].clone(),
        pos: map.len() + 3,
    };
    let mut rock_cnt: usize = 0;

    let limit: usize = 1_000_000_000_000;

    let mut first_cycle_detected = HashMap::new();
    let mut cycle_map_growth = None;
    let mut cycle_rocks= None;
    let mut cycle_input_state = None;
    for (i, dir) in input.iter().cloned().cycle().enumerate() {
        rock.push(&map, dir);

        if !rock.fall(&mut map) {
            rock_cnt+= 1;

            cur_shape_id = (cur_shape_id + 1) % shapes.len();
            rock = Rock {
                shape: shapes[cur_shape_id].clone(),
                pos: map.len() + 3,
            };
            if i > 2*input.len() {
                match first_cycle_detected.get(&(i%input.len(), cur_shape_id)) {
                    None => {
                        first_cycle_detected.insert((i% input.len(), cur_shape_id),(rock_cnt, map.len()));
                    },
                    Some((first_rock_cnt, first_map_len)) => {
                        cycle_map_growth = Some(map.len() - first_map_len);
                        cycle_rocks = Some(rock_cnt - first_rock_cnt);
                        cycle_input_state = Some(i% input.len());
                        break;
                    }
                }
            }
            if rock_cnt == limit {
                println!("{}", map.len());
                return Ok(());
            }
        }
    }

    let cycle_rocks = cycle_rocks.unwrap();
    let cycle_map_growth = cycle_map_growth.unwrap();
    let cycle_input_state = cycle_input_state.unwrap();
    let num_of_cycles = (limit-rock_cnt) / cycle_rocks;
    rock_cnt += cycle_rocks * num_of_cycles;
    let extra_map_growth = cycle_map_growth * num_of_cycles;

    if rock_cnt == limit {
        println!("{}", map.len()+extra_map_growth);
        return Ok(());
    }

    for dir in input.into_iter().cycle().skip(cycle_input_state+1) {
        rock.push(&map, dir);

        if !rock.fall(&mut map) {
            rock_cnt+= 1;

            if rock_cnt == limit {
                println!("{}", map.len()+extra_map_growth);
                return Ok(());
            }
            cur_shape_id = (cur_shape_id + 1) % shapes.len();
            rock = Rock {
                shape: shapes[cur_shape_id].clone(),
                pos: map.len() + 3,
            };
        }
    }

    Ok(())
}
