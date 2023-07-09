pub mod container;
pub mod error;

use crate::common::error::Error;
/// A custom result enum to handle Errors.
///
/// This is made with an intention of handling runtime
/// problems, instead of panic_abort within the
/// application
#[derive(Debug, Clone)]
pub enum Result<T> {
    /// Default Rust Result
    None,
    /// Result showing some process is complete and executed with no
    /// issues
    Ok(T),
    /// Result returning issues in character
    Error(Error),
}

impl<T> Result<T> {
    pub fn unwrap(&mut self) -> &T {
        match self {
            Result::Ok(value) => value,
            Result::Error(ref err_value) => {
                println!("{}", err_value);
                panic!()
            }
            _ => {
                panic!()
            }
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Result::Ok(value) => value,
            Result::Error(ref err_value) => {
                println!("{}", err_value);
                panic!()
            }
            _ => {
                panic!()
            }
        }
    }
}
