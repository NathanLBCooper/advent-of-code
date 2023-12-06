use crate::calibration::*;

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
    use common::core::Error;
    use common::core::Result;
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
        let first_digit = match find_first_digit(line) {
            Some(v) => v,
            None => return Err(Error::new("can't find first digit")),
        };

        let second_digit = match find_last_digit(line) {
            Some(v) => v,
            None => return Err(Error::new("can't find second digit")),
        };

        return Ok((first_digit * 10) + second_digit);
    }

    fn read_lines(filename: &str) -> Result<Vec<String>> {
        return match read_to_string(filename) {
            Ok(s) => return Ok(s.lines().map(String::from).collect()),
            Err(_) => Err(Error::new("can't read file")),
        };
    }

    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn find_first_digit(line: &str) -> Option<i32> {
        for i in 0..line.chars().count() {
            let remaining_text = &line[i..];

            let leading_char = remaining_text.chars().nth(0).unwrap();
            if leading_char.is_digit(10) {
                let digit = leading_char.to_digit(10).unwrap() as i32;
                return Some(digit);
            }

            for (j, number) in NUMBERS.iter().enumerate() {
                let number_length = number.chars().count();
                let remaining_length = remaining_text.chars().count();
                if number_length > remaining_length {
                    continue;
                }

                let text_to_compare = remaining_text[..number_length].to_string();

                if number.eq(&text_to_compare) {
                    return Some((j + 1) as i32);
                }
            }
        }

        return None;
    }

    // todo repetative
    fn find_last_digit(line: &str) -> Option<i32> {
        for i in 0..line.chars().count() {
            let remaining_text = &line[..line.chars().count() - i];

            let trailing_char = remaining_text.chars().last().unwrap();
            if trailing_char.is_digit(10) {
                let digit = trailing_char.to_digit(10).unwrap() as i32;
                return Some(digit);
            }

            for (j, number) in NUMBERS.iter().enumerate() {
                let number_length = number.chars().count();
                let remaining_length = remaining_text.chars().count();
                if number_length > remaining_length {
                    continue;
                }

                let text_to_compare =
                    remaining_text[remaining_text.chars().count() - number_length..].to_string();

                if number.eq(&text_to_compare) {
                    return Some((j + 1) as i32);
                }
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::calibration;

    #[test]
    fn can_calculate_sum_of_part_one_example_document() {
        let example_file = String::from("./example_calibration_part_one.txt");

        let result = calibration::calculate_sum_of_calibration_document(&example_file);

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn can_calculate_sum_of_part_two_example_document() {
        let example_file = String::from("./example_calibration_part_two.txt");

        let result = calibration::calculate_sum_of_calibration_document(&example_file);

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), 281);
    }
}
