// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
use anyhow::Result;
use std::collections::BTreeSet;
use std::io::BufRead;
use multimap::MultiMap;

enum Dir {
    N,
    S,
    W,
    E,
}
use Dir::*;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut elves = BTreeSet::new();
    for (y, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let dirs = [N, S, W, E];
    let mut cur_dir = 0;

    let mut moves = MultiMap::new();
    // dbg!(&elves);
    let initial = elves.len();
    for turn in 1.. {
        for (x, y) in &elves {
            let (x, y) = (*x, *y);
            if !elves.contains(&(x - 1, y - 1))
                && !elves.contains(&(x, y - 1))
                && !elves.contains(&(x + 1, y - 1))
                && !elves.contains(&(x - 1, y))
                && !elves.contains(&(x + 1, y))
                && !elves.contains(&(x - 1, y+1))
                && !elves.contains(&(x, y+1))
                && !elves.contains(&(x + 1, y+1)) {
                    moves.insert((x,y), (x,y));
                    continue;
                }

            let mut proposal = (x,y);
            for i in 0..4 {
                match dirs[(cur_dir + i) % 4] {
                    N => {
                        if !elves.contains(&(x - 1, y - 1))
                            && !elves.contains(&(x, y - 1))
                            && !elves.contains(&(x + 1, y - 1))
                        {
                            proposal = (x, y-1);
                            break;
                        }
                    }
                    S => {
                        if !elves.contains(&(x - 1, y + 1))
                            && !elves.contains(&(x, y + 1))
                            && !elves.contains(&(x + 1, y + 1))
                        {
                            proposal = (x, y+1);
                            break;
                        }
                    }
                    W => {
                        if !elves.contains(&(x - 1, y - 1))
                            && !elves.contains(&(x - 1, y))
                            && !elves.contains(&(x - 1, y + 1))
                        {
                            proposal = (x-1, y);
                            break;
                        }
                    }
                    E => {
                        if !elves.contains(&(x + 1, y - 1))
                            && !elves.contains(&(x + 1, y))
                            && !elves.contains(&(x + 1, y + 1))
                        {
                            proposal = (x+1, y);
                            break;
                        }
                    }
                }
            }
            moves.insert(proposal, (x, y));
        }

        let old_elves = elves;
        elves = BTreeSet::new();

        for (key, elems) in moves.iter_all() {
            if elems.len() == 1 {
                elves.insert(key.clone());
            } else {
                for elem in elems {
                    elves.insert(elem.clone());
                }
            }
        }

        moves.clear();
        cur_dir += 1;

        if elves == old_elves {
            println!("{turn}");
            break;
        }
        assert!(elves.len() == initial);
    }

    // let (mut maxx, mut maxy, mut minx, mut miny) = (i64::MIN, i64::MIN, i64::MAX, i64::MAX);
    // for (x, y) in elves.iter() {
    //     maxx = maxx.max(*x);
    //     maxy = maxy.max(*y);
    //     minx = minx.min(*x);
    //     miny = miny.min(*y);
    // }

    // // dbg!(minx, maxx, miny, maxy);
    // let res = (maxx-minx +1) * (maxy - miny +1) - (elves.len() as i64);
    // println!("{res}");
    Ok(())
}
