use std::{error, fmt};

#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidLength(usize),
    InvalidCode(String),
    InvalidCoordinate(Vec<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidLength(length) => {
                write!(f, "Invalid length {}, should be 2, 4, 6, 8 or 10", length)
            }
            Error::InvalidCode(code) => write!(f, "Invalid code: {}", code),
            Error::InvalidCoordinate(coordinate) => {
                write!(f, "Invalid coordinate: {:?}", coordinate)
            }
        }
    }
}
impl error::Error for Error {}

pub fn min(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    }
    b
}

pub fn max(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}

pub const DIGITS: &'static str = "23456789CFGHJMPQRVWX";
