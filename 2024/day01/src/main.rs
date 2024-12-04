use std::num::ParseIntError;
use common::core::AdventError;
use common::file::read_lines;

fn main() {
    let lines = match parse_file() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e.message());
            std::process::exit(1)
        }
    };

    let (mut first_numbers, mut second_numbers): (Vec<i32>, Vec<i32>) = lines.iter().map(|(f, s)| (*f, *s)).unzip();

    first_numbers.sort();
    second_numbers.sort();

    let mut total_distance = 0;
    for i in 0..lines.len() {
        let distance = (first_numbers[i] - second_numbers[i]).abs();
        total_distance += distance;
    }

    println!("Total distance: {}", total_distance);
}

fn parse_file() -> Result<Vec<(i32, i32)>, AdventError> {
    let lines = read_lines("./input.txt")?;
    let numbers = match lines.iter().map(|l| parse_line(l)).collect() {
        Ok(s) => s,
        Err(e) => return Err(AdventError::ParseIntError { inner: e }),
    };

    Ok(numbers)
}

fn parse_line(line: &str) -> Result<(i32, i32), ParseIntError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let first = parts[0].parse::<i32>()?;
    let second = parts[1].parse::<i32>()?;

    Ok((first, second))
}