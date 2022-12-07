// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use text_io::try_scan;

#[derive(Clone, Debug)]
enum LsItem {
    File(String, usize),
    Dir(String),
}

struct Ls(Vec<LsItem>);
struct Cd(String);

fn parse_cd(s: &str) -> Result<Cd> {
    let l = s.lines().next().ok_or_else(|| anyhow!("no lines"))?;
    let to: String;
    try_scan!(l.trim().bytes() => "cd {}", to);
    Ok(Cd(to))
}

fn parse_ls(s: &str) -> Result<Ls> {
    let mut lines = s.lines();
    let mut listing = Vec::new();
    let l = lines.next().ok_or_else(|| anyhow!("no lines"))?;
    if l.trim() != "ls" {
        return Err(anyhow!("not ls"));
    }
    for l in lines {
        if l.trim().len() > 1 {
            let mut words = l.split(" ");
            let size_or_dir = words.next().unwrap();
            let name = words.next().unwrap().to_owned();
            if size_or_dir == "dir" {
                listing.push(LsItem::Dir(name));
            } else {
                let size = size_or_dir.parse::<usize>().unwrap();
                listing.push(LsItem::File(name, size));
            }
        }
    }
    Ok(Ls(listing))
}

fn walk(
    tree: &HashMap<String, Vec<LsItem>>,
    result: &mut HashMap<String, usize>,
    path: &PathBuf,
) -> usize {
    let mut dir_size = 0;
    for item in tree.get(path.to_str().unwrap()).unwrap() {
        match item {
            LsItem::Dir(name) => {
                dir_size = dir_size + walk(tree, result, &path.join(name));
            }
            LsItem::File(_name, size) => {
                dir_size = dir_size + size;
            }
        };
    }
    result.insert(path.to_str().unwrap().to_owned(), dir_size);
    dir_size
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut s = String::new();
    stdin.lock().read_to_string(&mut s).expect("Couldn't read");
    let mut cwd = PathBuf::new();
    let mut dir_tree = HashMap::new();
    for command in s.split("$") {
        if let Ok(Cd(path)) = parse_cd(command) {
            match path.as_str() {
                ".." => {
                    cwd.pop();
                }
                "/" => {
                    cwd.clear();
                    cwd.push("/");
                }
                path => {
                    cwd.push(path);
                }
            }
        } else if let Ok(Ls(listing)) = parse_ls(command) {
            dir_tree.insert(cwd.to_str().unwrap().to_owned(), listing);
        } else {
            ()
        }
    }
    let mut cwd = PathBuf::new();
    cwd.push("/");
    let mut result = HashMap::new();
    let _ = walk(&dir_tree, &mut result, &cwd);

    let mut sum = 0;
    for val in result.values() {
        if *val <= 100000 {
            sum += val;
        }
    }
    println!("{}", sum);
    Ok(())
}
