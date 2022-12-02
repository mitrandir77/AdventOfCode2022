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
    while let Ok((first, result)) = read_pair() {
        let first = first as i32 - 'A' as i32;
        let result = match result {
            'X' => 2,
            'Y' => 0,
            'Z' => 1,
            _ => panic!("wrong difference"),
        };
        let second = (first + result) % 3;

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
