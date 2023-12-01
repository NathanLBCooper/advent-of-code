use crate::calibration::calculate_sum_of_calibration_document;

fn main() {
    let result = calculate_sum_of_calibration_document(&String::from("./calibration.txt"));

    match result {
        Ok(value) => {
            println!("Sum of calibrations: {}", value)
        }
        Err(err) => {
            println!("Failed {}", err.get_reason())
        }
    }
}

mod calibration {
    use crate::core::Error;
    use crate::core::Result;
    use std::fs::read_to_string;

    pub fn calculate_sum_of_calibration_document(filename: &str) -> Result<i32> {
        let lines = match read_lines(&filename) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let line_values: Vec<i32> = match lines.iter().map(|l| sum_line(&l)).collect() {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let total_sum = line_values.iter().sum();

        return Ok(total_sum);
    }

    fn sum_line(line: &str) -> Result<i32> {
        let is_digit = |c: &char| c.is_digit(10);

        let first_digit = match line.chars().find(is_digit) {
            Some(v) => v.clone(),
            None => return Err(Error::new("can't find first digit")),
        };

        let second_digit = match line.chars().rfind(is_digit) {
            Some(v) => v.clone(),
            None => return Err(Error::new("can't find second digit")),
        };

        let appended_digits: String = [first_digit, second_digit].iter().collect();

        let sum: i32 = appended_digits.parse::<i32>().unwrap();
        return Ok(sum);
    }

    fn read_lines(filename: &str) -> Result<Vec<String>> {
        return match read_to_string(filename) {
            Ok(s) => return Ok(s.lines().map(String::from).collect()),
            Err(_) => Err(Error::new("can't read file")),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::calibration::*;

    #[test]
    fn can_calculate_sum_of_part_one_example_document() {
        let example_file = String::from("./example_calibration_part_one.txt");

        let result = calculate_sum_of_calibration_document(&example_file);

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn can_calculate_sum_of_part_two_example_document() {
        let example_file = String::from("./example_calibration_part_two.txt");

        let result = calculate_sum_of_calibration_document(&example_file);

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), 281);
    }
}

mod core {
    use std::{fmt::Debug, result};

    pub type Result<T> = result::Result<T, Error>;

    pub struct Error {
        reason: String,
    }

    impl Error {
        pub fn new(reason: &str) -> Self {
            return Self {
                reason: String::from(reason),
            };
        }

        pub fn get_reason(&self) -> &String {
            return &self.reason;
        }
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Error")
                .field("reason", &self.reason)
                .finish()
        }
    }
}
