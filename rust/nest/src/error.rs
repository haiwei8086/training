
use std::{io, fmt, convert};
use std::error::Error;


#[derive(Debug)]
pub enum NsError {
    IO(io::Error),
    Unknow
}

impl fmt::Display for NsError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NsError::IO(ref err) => err.fmt(f),
            _ => write!(f, "Unknow error!"),
        }
    }
}

impl Error for NsError {

    fn description(&self) -> &str {
        match *self {
            NsError::IO(ref err) => err.description(),
            _ => "Unknow error!",
        }
    }
}

impl convert::From<io::Error> for NsError {

    fn from(err: io::Error) -> NsError {
        NsError::IO(err)
    }
}
