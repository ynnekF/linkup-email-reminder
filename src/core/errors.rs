use std::fmt;

pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub struct CliError {
    pub message: String,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        CliError { message }
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        CliError {
            message: message.to_string(),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError {
            message: error.to_string(),
        }
    }
}

impl From<csv::Error> for CliError {
    fn from(error: csv::Error) -> Self {
        CliError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        CliError {
            message: error.to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for CliError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        CliError {
            message: error.to_string(),
        }
    }
}
