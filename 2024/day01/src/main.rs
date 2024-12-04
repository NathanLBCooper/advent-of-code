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

    let (first_numbers, second_numbers): (Vec<i32>, Vec<i32>) = lines.iter().map(|(f, s)| (*f, *s)).unzip();

    println!("Total distance: {}", part_one(first_numbers.as_slice(), second_numbers.as_slice(), lines.len()));
    println!("Similarity: {}", part_two(first_numbers.as_slice(), second_numbers.as_slice()));
}

fn part_one(first_numbers: &[i32], second_numbers: &[i32], length: usize) -> i32 {
    let mut sorted_first_numbers = first_numbers.to_vec();
    sorted_first_numbers.sort();
    let mut sorted_second_numbers = second_numbers.to_vec();
    sorted_second_numbers.sort();

    let mut total_distance = 0;
    for i in 0..length {
        let distance = (sorted_first_numbers[i] - sorted_second_numbers[i]).abs();
        total_distance += distance;
    }

    total_distance
}

fn part_two(first_numbers: &[i32], second_numbers: &[i32]) -> i32 {
    let mut second_number_counts = std::collections::HashMap::new();
    for i in second_numbers {
        let count = second_number_counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut similarity = 0;
    for i in first_numbers {
        match second_number_counts.get(i) {
            None => {}
            Some(c) => { similarity += i * c }
        }
    }

    similarity
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