use types::FromStrError;
use std::error::Error as IoError;
#[derive(Debug)]
pub enum Error {
    IO(Box<IoError>),
    HEX(FromStrError),
}
