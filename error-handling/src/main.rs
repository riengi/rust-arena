// Error handling
// Using anyhow create - allowing return error of any type implementing std::error:Error triat
// Using thiserror crate - allowing nice error messages

use std::{
    fs::File,
    process::{ExitCode, Termination},
};
mod results;

// Custom Errors
#[derive(thiserror::Error, Debug, Clone)]
pub enum MyError {
    #[error("Internal malfunction")]
    Internal,
    #[error("Other issue")]
    Other,
    #[error("Input/Output Error, detail: {0}")]
    InputOutput(String),
}

// Implement custom exit codes via Termination trait
impl Termination for MyError {
    fn report(self) -> ExitCode {
        match self {
            MyError::Internal => ExitCode::from(1),
            MyError::Other => ExitCode::from(255),
            MyError::InputOutput(_) => ExitCode::from(3),
        }
    }
}

// Conversion from std::io::Error
impl std::convert::From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::InputOutput(err.kind().to_string())
    }
}

// returning MyError
fn some_function() -> Result<(), MyError> {
    return Err(MyError::Internal);
}

// This one returns original IO error
fn open_file() -> Result<(), std::io::Error> {
    // returns Result<File, std::io::Error
    File::open("foo.txt")?;
    Ok(())
}

// This one returns MyError
fn open_file_own_error() -> Result<(), MyError> {
    // returns Result<File, std::io::Error
    File::open("foo.txt")?;
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    results::execute();

    open_file_own_error()?;
    open_file()?;
    some_function()?;
    Ok(())
}
