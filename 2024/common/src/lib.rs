pub mod core {
    use std::{fmt::Debug};
    use std::io;

    #[derive(Debug)]
    pub enum AdventError {
        GeneralError { message: String },
        IoError { inner: io::Error },
        ParseIntError { inner: std::num::ParseIntError },
    }

    impl AdventError {
        pub fn message(&self) -> String {
            match self {
                AdventError::GeneralError { message } => message.clone(),
                AdventError::IoError { inner } => { inner.to_string() }
                AdventError::ParseIntError { inner } => { inner.to_string() }
            }
        }
    }
}

pub mod file {
    use std::fs::read_to_string;
    use crate::core::AdventError;

    pub fn read_lines(filename: &str) -> Result<Vec<String>, AdventError> {
        match read_to_string(filename) {
            Ok(s) => Ok(s.lines().map(String::from).collect()),
            Err(e) => Err(AdventError::IoError { inner: e }),
        }
    }
}
