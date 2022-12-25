use anyhow::Result;
use radix_fmt::radix;
use std::io::BufRead;

fn from_snafu(digits: &str) -> Result<i64> {
    let number: String = digits
        .chars()
        .map(|c| match c {
            '=' => '0',
            '-' => '1',
            '0' => '2',
            '1' => '3',
            '2' => '4',
            _ => {
                panic!("character is not a SNAFU digit")
            }
        })
        .collect();
    let mut num = i64::from_str_radix(&number, 5)?;

    let mut offset = -2;
    for _i in 0..number.len() {
        num += offset;
        offset *= 5;
    }
    Ok(num)
}

fn to_snafu(mut num: i64) -> Result<String> {
    let digits = radix(num, 5).to_string();

    let mut offset = 2;
    let len = digits.len();
    for _i in 0..digits.len() {
        num += offset;
        offset *= 5;
    }

    let digits = radix(num, 5).to_string();
    let leave_alone = digits.len() - len;

    let number: String = digits
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if leave_alone > i {
                return c;
            }

         match c {
            '0' => '=',
            '1' => '-',
            '2' => '0',
            '3' => '1',
            '4' => '2',
            _ => {
                panic!("character is not a base5 digit")
            }
}})
        .collect();
    Ok(number)
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let num = from_snafu(&line)?;
        sum += num;

    }

    println!("{}", to_snafu(sum)?);
    Ok(())
}
