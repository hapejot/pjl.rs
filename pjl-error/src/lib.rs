use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct AppError {
    msg: String,
}

impl AppError {
    pub fn new<S>(msg: S) -> Self where S: Into<String> {
        Self { msg: msg.into() }
    }
    
    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl<E> From<E> for AppError
where
    E: Display,
{
    fn from(value: E) -> Self {
        AppError {
            msg: value.to_string(),
        }
    }
}

// is not working, since there is a conflicting core definition for this then.
// impl Display for AppError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// is not working because we cannot implement Display....
// impl std::error::Error for AppError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         None
//     }

//     fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn io_err() -> Result<(), std::io::Error> {
        Err(std::io::Error::from_raw_os_error(10))
    }

    fn err() -> Result<(), AppError> {
        io_err()?;
        Ok(())
    }

    #[test]
    fn create() {
        match err() {
            Ok(_) => todo!(),
            Err(e) => {
                assert!(e.message().contains("(os error 10)"))
            }
        }
    }
}
