#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

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

// fn digit_to_value(x: char) -> usize {
//     DIGITS.chars().position(|c| c == x).unwrap()
// }
