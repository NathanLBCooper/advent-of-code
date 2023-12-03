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
