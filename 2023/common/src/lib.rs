pub mod core {
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

    pub fn into_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
        v.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }

    pub trait IntoArr<T> {
        fn into_arr<const N: usize>(self) -> [T; N];
    }

    impl<T> IntoArr<T> for Vec<T> {
        fn into_arr<const N: usize>(self) -> [T; N] {
            self.try_into().unwrap_or_else(|v: Vec<T>| {
                panic!("Expected a Vec of length {} but it was {}", N, v.len())
            })
        }
    }
}

pub mod file {
    use crate::core::Error;
    use crate::core::Result;
    use std::fs::read_to_string;

    pub fn read_lines(filename: &str) -> Result<Vec<String>> {
        return match read_to_string(filename) {
            Ok(s) => return Ok(s.lines().map(String::from).collect()),
            Err(_) => Err(Error::new("can't read file")),
        };
    }
}

pub mod parsing {
    use std::fmt::Debug;
    use std::str::FromStr;

    pub fn parse_numbers<T>(numbers_str: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        numbers_str
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod parsing_tests {
    use crate::parsing::*;

    #[test]
    fn can_parse_line_of_numbers() {
        let line = "79 14 55 13";

        let numbers = parse_numbers::<i32>(&line);

        assert_eq!(numbers.iter().count(), 4);
        let mut iter = numbers.iter();
        assert_eq!(iter.next().unwrap().clone(), 79);
        assert_eq!(iter.next().unwrap().clone(), 14);
        assert_eq!(iter.next().unwrap().clone(), 55);
        assert_eq!(iter.next().unwrap().clone(), 13);
    }
}

mod geometry {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub struct Coordinate {
        pub x: i64,
        pub y: i64,
    }

    pub fn calculate_path_distance(left: &Coordinate, right: &Coordinate) -> i64 {
        let x_distance = i64::abs(right.x - left.x);
        let y_distance = i64::abs(right.y - left.y);

        // let non_diagonal_component = i64::abs(x_distance - y_distance);
        // let diagonal_component = i64::max(x_distance, y_distance) - non_diagonal_component;

        // non_diagonal_component + diagonal_component

        x_distance + y_distance
    }
}

#[cfg(test)]
mod geometry_tests {
    use crate::geometry::{calculate_path_distance, Coordinate};
    use test_case::test_case;

    #[test_case(Coordinate { x: 5, y: 7 }, Coordinate { x: 5, y: 7 }, 0; "itself")]
    #[test_case(Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 1 }, 1; "1 down")]
    #[test_case(Coordinate { x: 0, y: 2 }, Coordinate { x: 0, y: 1 }, 1; "1 up")]
    #[test_case(Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 0 }, 1; "1 right")]
    #[test_case(Coordinate { x: 2, y: 0 }, Coordinate { x: 1, y: 0 }, 1; "1 left")]
    #[test_case(Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 1 }, 2; "1 down and right")]
    #[test_case(Coordinate { x: 2, y: 2 }, Coordinate { x: 0, y: 0 }, 4; "2 up and left")]
    fn can_get_distance_between_coordinates(left: Coordinate, right: Coordinate, expected: i64) {
        let distance = calculate_path_distance(&left, &right);

        assert_eq!(distance, expected);
    }
}
