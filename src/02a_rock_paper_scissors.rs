// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use text_io::try_scan;


fn read_pair () -> Result<(char, char)>{
    let first: char;
    let second: char;
    try_scan!("{} {}", first, second);
    Ok((first, second))
}

fn main() -> Result<()> {
    let mut points = 0;
    while let Ok((first, second)) = read_pair() {
        let first = first as i32 - 'A' as i32;
        let second = second as i32 - 'X' as i32;
        let result = (second - first + 3) % 3;
        points = points + second + 1;
        points = points
            + match result {
                0 => 3,
                1 => 6,
                2 => 0,
                _ => panic!("wrong difference"),
            };
    }
    println!("{}", points);
    Ok(())
}
